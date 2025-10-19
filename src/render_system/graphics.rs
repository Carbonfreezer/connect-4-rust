//! This module contains everything to drawing boards, stones and simple ui elements.

use crate::board_logic::bit_board::BitBoard;
use crate::board_logic::bit_board_coding::{BOARD_HEIGHT, BOARD_WIDTH};
use crate::{debug_check_board_coordinates, debug_check_draw_coordinates};
use glume::gl;
use glume::gl::types::*;

/// Represents color types we can draw elements with.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Brown,
    Yellow,
    LightYellow,
    Blue,
    LightBlue,
    Grey
}

/// Generates an RGB value for any of the Colors indicated.
fn get_color_vector(color: Color) -> [f32; 3] {
    match color {
        Color::Brown => [0.48, 0.25, 0.0],
        Color::Yellow => [0.85, 0.65, 0.12],
        Color::LightYellow => [1.0, 0.91, 0.0],
        Color::Blue => [0.0, 0.28, 0.67],
        Color::LightBlue => [0.0, 0.58, 1.0],
        Color::Grey => [0.01, 0.19, 0.12],
    }
}

/// The graphics painter is capable of painting circles and rectangles. It is also capable of stenceling
/// out circles of rectangles. The circle stencils have to be painted upfront.
pub struct GraphicsPainter {
    shader_program: GLuint,
    circle_vba: GLuint,
    num_circle_vertices: GLint,
    rectangle_vba: GLuint,
    num_rectangle_vertices: GLint,
    translation: GLint,
    color: GLint,
    scale: GLint,
}

impl GraphicsPainter {
    pub fn new() -> GraphicsPainter {
        let shader_program = Self::create_shader_program();
        let (scale, translation, color) = Self::get_shader_constants(shader_program);
        let (circle_vba, num_circle_vertices) = Self::create_circle_vba();
        let (rectangle_vba, num_rectangle_vertices) = Self::create_rectangle_vba();

        GraphicsPainter {
            shader_program,
            circle_vba,
            num_circle_vertices,
            rectangle_vba,
            num_rectangle_vertices,
            translation,
            scale,
            color,
        }
    }

    fn compile_shader(source: &str, shader_type: GLenum) -> GLuint {
        let shader = unsafe { gl::CreateShader(shader_type) };
        let c_str = std::ffi::CString::new(source).unwrap();
        unsafe {
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);
        }
        shader
    }

    fn create_shader_program() -> GLuint {
        let v_code = r#"
            #version 330
            uniform vec2 translation;
            uniform vec2 scale;
            layout(location = 0) in vec2 position;
            void main()
            {
	            gl_Position = vec4((position * scale) + translation,  0.0,  1.0);
            }
            "#;

        let f_code = r#"
            #version 330
            uniform vec3 PaintColor;
            out vec4 color;
            void main()
            {
                color = vec4(PaintColor, 1.0);
            }
            "#;

        let v_shader = Self::compile_shader(v_code, gl::VERTEX_SHADER);
        let f_shader = Self::compile_shader(f_code, gl::FRAGMENT_SHADER);

        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, v_shader);
            gl::AttachShader(program, f_shader);
            gl::LinkProgram(program);
            gl::DetachShader(program, v_shader);
            gl::DetachShader(program, f_shader);
            gl::DeleteShader(v_shader);
            gl::DeleteShader(f_shader);

            program
        }
    }

    fn get_shader_constants(program: GLuint) -> (GLint, GLint, GLint) {
        let color_str = std::ffi::CString::new("PaintColor").unwrap();
        let translation_str = std::ffi::CString::new("translation").unwrap();
        let scale_str = std::ffi::CString::new("scale").unwrap();
        let scale: GLint;
        let translation;
        let color;
        unsafe {
            translation = gl::GetUniformLocation(program, translation_str.as_ptr());
            color = gl::GetUniformLocation(program, color_str.as_ptr());
            scale = gl::GetUniformLocation(program, scale_str.as_ptr());
        }

        (scale, translation, color)
    }

    /// Helper function to create a vbo and vba
    fn create_vba(vertex_data: &[f32]) -> GLuint {
        let mut vbo: u32 = 0;
        let mut vba: u32 = 0;

        unsafe {
            gl::CreateBuffers(1, &mut vbo);
            gl::CreateVertexArrays(1, &mut vba);

            gl::BindVertexArray(vba);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (size_of_val(vertex_data)) as GLsizeiptr,
                vertex_data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                (2 * size_of::<f32>()) as GLint,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        vba
    }

    fn create_circle_vba() -> (GLuint, GLint) {
        const POINTS_IN_CIRCLE: usize = 100;

        let mut vertices: Vec<f32> = Vec::with_capacity(POINTS_IN_CIRCLE * 2);

        for i in 0..POINTS_IN_CIRCLE {
            let angle = i as f32 * 2.0 * std::f32::consts::PI / POINTS_IN_CIRCLE as f32;
            vertices.push(angle.cos());
            vertices.push(angle.sin());
        }

        (Self::create_vba(&vertices), POINTS_IN_CIRCLE as i32)
    }

    fn create_rectangle_vba() -> (GLuint, GLint) {
        let vertices: [f32; 8] = [-0.5, -0.5, 0.5, -0.5, 0.5, 0.5, -0.5, 0.5];
        (Self::create_vba(&vertices), 4)
    }

    fn draw_geometry(
        &self,
        vba: GLuint,
        num_vertices: GLint,
        scale: [f32; 2],
        translation: [f32; 2],
        color: Color,
    ) {
        let color_vec = get_color_vector(color);

        unsafe {
            gl::UseProgram(self.shader_program);
            gl::BindVertexArray(vba);
            gl::Uniform3fv(self.color, 1, color_vec.as_ptr());
            gl::Uniform2fv(self.translation, 1, translation.as_ptr());
            gl::Uniform2fv(self.scale, 1, scale.as_ptr());
            gl::DrawArrays(gl::TRIANGLE_FAN, 0, num_vertices);
        }
    }

    /// Draws a circle to be visible on screen.
    pub fn draw_circle_normal(&self, radius: f32, position: [f32; 2], color: Color) {
        debug_check_draw_coordinates!(position);
        let scale = [radius, radius];
        self.draw_geometry(
            self.circle_vba,
            self.num_circle_vertices,
            scale,
            position,
            color,
        );
    }

    /// Draws the circle only into the stencil buffer. This is meant to be used in conjunction
    /// with the *draw_rectangle_conditional_stencil*, that skips drawing the rectangle, where the
    /// mask has been drawn.
    fn draw_circle_into_stencil(&self, radius: f32, position: [f32; 2]) {
        debug_check_draw_coordinates!(position);
        let scale = [radius, radius];

        // Draw into stencil only.
        unsafe {
            gl::StencilMask(0xff);
            gl::ColorMask(gl::FALSE, gl::FALSE, gl::FALSE, gl::FALSE);
            gl::StencilOp(gl::INCR, gl::INCR, gl::INCR);
        }
        self.draw_geometry(
            self.circle_vba,
            self.num_circle_vertices,
            scale,
            position,
            Color::Brown,
        );
        unsafe {
            gl::StencilMask(0);
            gl::ColorMask(gl::TRUE, gl::TRUE, gl::TRUE, gl::TRUE);
            gl::StencilOp(gl::KEEP, gl::KEEP, gl::KEEP);
        }
    }

    /// Draws a rectangle with the two corners (and color) given.
    pub fn draw_rectangle_normal(&self, lower_left: [f32; 2], upper_right: [f32; 2], color: Color) {
        debug_check_draw_coordinates!(lower_left);
        debug_check_draw_coordinates!(upper_right);
        let translation = [
            (lower_left[0] + upper_right[0]) / 2.0,
            (lower_left[1] + upper_right[1]) / 2.0,
        ];
        let scale = [
            upper_right[0] - lower_left[0],
            upper_right[1] - lower_left[1],
        ];

        self.draw_geometry(
            self.rectangle_vba,
            self.num_rectangle_vertices,
            scale,
            translation,
            color,
        );
    }

    /// Draws a rectangle with the two corners but only at the positions where the stencil is not set.
    /// This is meant to be used with *draw_circle_into_stencil*.
    fn draw_rectangle_conditional_stencil(
        &self,
        lower_left: [f32; 2],
        upper_right: [f32; 2],
        color: Color,
    ) {
        let translation = [
            (lower_left[0] + upper_right[0]) / 2.0,
            (lower_left[1] + upper_right[1]) / 2.0,
        ];
        let scale = [
            upper_right[0] - lower_left[0],
            upper_right[1] - lower_left[1],
        ];

        unsafe {
            gl::StencilFunc(gl::EQUAL, 0, 0xff);
        }
        self.draw_geometry(
            self.rectangle_vba,
            self.num_rectangle_vertices,
            scale,
            translation,
            color,
        );

        unsafe {
            gl::StencilFunc(gl::ALWAYS, 0, 0xff);
        }
    }

    /// The radius with which we want to draw the stones in the below function.
    pub const CIRCLE_RADIUS: f32 = 1.0 / BOARD_WIDTH as f32 * 0.8;

    /// Returns the drawing coordinates for an indicated stone position.
    pub fn get_drawing_coordinates(x_stone: usize, y_stone: usize) -> [f32; 2] {
        debug_check_board_coordinates!(x_stone, y_stone);
        [
            (x_stone as f32 / BOARD_WIDTH as f32) * 2.0 - 1.0 + 1.0 / BOARD_WIDTH as f32,
            (y_stone as f32 / BOARD_WIDTH as f32) * 2.0 - 1.0 + 1.0 / BOARD_WIDTH as f32,
        ]
    }

    /// Gets a painting position above the column.
    pub fn get_drawing_coordinates_above_column(column: usize) -> [f32; 2] {
        debug_check_board_coordinates!(col: column);
        [
            (column as f32 / BOARD_WIDTH as f32) * 2.0 - 1.0 + 1.0 / BOARD_WIDTH as f32,
            (6.0 / BOARD_WIDTH as f32) * 2.0 - 1.0 + 1.0 / BOARD_WIDTH as f32,
        ]
    }

    /// Renders the board as is with all the stones in there.
    pub fn render_board(&self, board: &BitBoard) {
        // First we draw the stencil circles.
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                debug_check_board_coordinates!(x, y);
                self.draw_circle_into_stencil(
                    Self::CIRCLE_RADIUS,
                    Self::get_drawing_coordinates(x, y),
                );
            }
        }

        self.draw_rectangle_conditional_stencil(
            [-1.0, -1.0],
            [1.0, 1.0 - 2.0 / BOARD_WIDTH as f32],
            Color::Brown,
        );

        for (x, y, first) in board.get_board_positioning() {
            debug_check_board_coordinates!(x, y);
            let color = if first { Color::Yellow } else { Color::Blue };
            self.draw_circle_normal(
                Self::CIRCLE_RADIUS,
                Self::get_drawing_coordinates(x, y),
                color,
            );
        }
    }

    /// Renders the indicated stones into the stone array with highlighted color. Indicates
    /// if this is the first player who is winning to pick the right color.
    pub fn render_winning_stones(
        &self,
        is_first_player_winning: bool,
        list_of_positions: &Vec<(usize, usize)>,
    ) {
        let color = if is_first_player_winning {
            Color::LightYellow
        } else {
            Color::LightBlue
        };

        for (column, row) in list_of_positions {
            self.draw_circle_normal(
                Self::CIRCLE_RADIUS,
                Self::get_drawing_coordinates(*column, *row),
                color,
            );
        }
    }

    /// Draws the stone at the indicated coordinates, this is meant for drawing an animated stone.
    pub fn draw_stone_at_coordinates(&self, position: [f32; 2], is_first_player: bool) {
        debug_check_draw_coordinates!(position);
        self.draw_circle_normal(
            Self::CIRCLE_RADIUS,
            position,
            if is_first_player {
                Color::Yellow
            } else {
                Color::Blue
            },
        )
    }
}
