//! This module contains everything to drawing boards, stones and simple ui elements.

use crate::board_logic::bit_board::BitBoard;
use crate::board_logic::bit_board_coding::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::debug_check_board_coordinates;
use macroquad::prelude::*;

/// The window dimension that will be used for rendering.
pub const WINDOW_DIMENSION: f32 = 700.0;

/// The radius with which we want to draw the stones in the below function.
pub const CIRCLE_RADIUS: f32 = WINDOW_DIMENSION / BOARD_WIDTH as f32 * 0.8 * 0.5;

/// Represents color types we can draw elements with.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SymbolColor {
    Brown,
    Yellow,
    LightYellow,
    Blue,
    LightBlue,
}


/// Static array with colors that can be queried.
const COLOR_ARRAY: [Color; 5] = [
    Color::new(0.48, 0.25, 0.0, 1.0),
    Color::new(0.75, 0.55, 0.06, 1.0),
    Color::new(1.0, 0.91, 0.0, 1.0),
    Color::new(0.0, 0.28, 0.67, 1.0),
    Color::new(0.0, 0.58, 1.0, 1.0),
];

/// Generates an RGB value for any of the Colors indicated.
pub fn get_color(color: SymbolColor) -> &'static Color {
    match color {
        SymbolColor::Brown => &COLOR_ARRAY[0],
        SymbolColor::Yellow => &COLOR_ARRAY[1],
        SymbolColor::LightYellow => &COLOR_ARRAY[2],
        SymbolColor::Blue => &COLOR_ARRAY[3],
        SymbolColor::LightBlue => &COLOR_ARRAY[4],
    }
}

/// Returns the drawing coordinates for an indicated stone position.
pub const fn get_drawing_coordinates(x_stone: u32, y_stone: u32) -> Vec2 {
    Vec2 {
        x: (x_stone as f32 + 0.5) * WINDOW_DIMENSION / BOARD_WIDTH as f32,
        y: (y_stone as f32 + 0.5) * WINDOW_DIMENSION / BOARD_WIDTH as f32,
    }
}

/// Gets a painting position above the column.
pub const fn get_drawing_coordinates_above_column(column: u32) -> Vec2 {
    // One column above the maximum.
    get_drawing_coordinates(column, 7)
}

/// Renders the board as is with all the stones in there.
pub fn render_board(board: &BitBoard, board_texture: &Texture2D) {
    draw_texture(board_texture, 0.0, 0.0, WHITE);

    for (x, y, first) in board.get_board_positioning() {
        debug_check_board_coordinates!(x, y);
        let color = if first {
            get_color(SymbolColor::Yellow)
        } else {
            get_color(SymbolColor::Blue)
        };
        let draw_pos = get_drawing_coordinates(x, y);
        draw_circle(draw_pos.x, draw_pos.y, CIRCLE_RADIUS, *color);
    }
}

/// Renders the indicated stones into the stone array with highlighted color. Indicates
/// if this is the first player who is winning to pick the right color.
pub fn render_winning_stones(is_first_player_winning: bool, list_of_positions: &Vec<(u32, u32)>) {
    let color = get_color(if is_first_player_winning {
        SymbolColor::LightYellow
    } else {
        SymbolColor::LightBlue
    });

    for (column, row) in list_of_positions {
        let draw_pos = get_drawing_coordinates(*column, *row);
        draw_circle(draw_pos.x, draw_pos.y, CIRCLE_RADIUS, *color);
    }
}

/// Draws the stone at the indicated coordinates, this is meant for drawing an animated stone.
pub fn draw_stone_at_coordinates(position: Vec2, is_first_player: bool) {
    let color = get_color(if is_first_player {
        SymbolColor::Yellow
    } else {
        SymbolColor::Blue
    });

    draw_circle(position.x, position.y, CIRCLE_RADIUS, *color);
}


/// A standardized way on how to write text in the game.
pub fn print_text(text: &str, position: Vec2) {
    draw_text_ex(
        text,
        position.x,
        position.y,
        TextParams {
            font: None,
            font_size: 50,
            font_scale: -1.0,
            font_scale_aspect: -1.0,
            rotation: 0.0,
            color: WHITE,
        },
    );
}


/// Creates an internal material for the offscreen texture of the game board.
/// Simply paints black with an alpha of zero and replaces the content.
fn create_cutout_material() -> Material {
    let vertex_shader = r#"#version 100
    attribute vec3 position;

    uniform mat4 Model;
    uniform mat4 Projection;

    void main() {
        gl_Position = Projection * Model * vec4(position, 1);
    }
    "#;

    let fragment_shader = r#"#version 100
    precision mediump float; 

    void main() {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
    }
    "#;

    load_material(
        ShaderSource::Glsl {
            vertex: vertex_shader,
            fragment: fragment_shader,
        },
        MaterialParams {
            pipeline_params: miniquad::PipelineParams {
                color_blend: Some(miniquad::BlendState::new(
                    miniquad::Equation::Add,
                    miniquad::BlendFactor::One,
                    miniquad::BlendFactor::Zero,
                )),
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .unwrap()
}

/// Creates the board texture with holes. Is done once and can then be reused for the remainder of the game.
pub fn create_board_texture() -> Texture2D {
    let board_height = WINDOW_DIMENSION * (6.0 / 7.0);
    let render_target = render_target(WINDOW_DIMENSION as u32, board_height as u32);
    render_target.texture.set_filter(FilterMode::Linear);

    // Set render target.
    let mut target_cam =
        Camera2D::from_display_rect(Rect::new(0.0, 0.0, WINDOW_DIMENSION, board_height));
    target_cam.render_target = Some(render_target.clone());
    set_camera(&target_cam);

    // 1. Draw board
    clear_background(*get_color(SymbolColor::Brown));

    // 2. Create cut out material
    let cutout_material = create_cutout_material();
    gl_use_material(&cutout_material);

    // 3. Create wholes
    for row in 0..6 {
        for col in 0..7 {
            let pos = get_drawing_coordinates(col, row);
            draw_circle(pos.x, pos.y, CIRCLE_RADIUS, WHITE);
        }
    }

    // 4. Back to  Standard-Material
    gl_use_default_material();

    // 5. Back to Standard Camera.
    set_default_camera();

    render_target.texture
}
