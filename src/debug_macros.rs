//! Here we collect a couple of custom debug macros.

/// Macro to check if the coordinates are in the -1..1 range.
/// And checks, that it is a 2d float vector.
///
/// # Example
/// ```
/// let x : [f32;2] = [-0.1, 0.2];
/// debug_check_draw_coordinates!(x);
/// ```
#[macro_export]
macro_rules! debug_check_draw_coordinates {
    ($coord:expr) => {
        // Forces f32 slice at compile time.
        let [x, y]: [f32; 2] = $coord;
        debug_assert!(
            (-1.0 <= x) && (x <= 1.0) && (-1.0 <= y) && (y <= 1.0),
            "Illegal coordinates: x={}, y={} must be between -1.0 and 1.0",
            x,
            y,
        );
    };
}

/// Verifier macros for coordinates, can be used with x and y coordinates for a position, or a
/// column only. Checks for the type to be usize and if they do not exceed the desired range,
///
/// # Example
/// ```
/// let x : usize = 2;
/// let y : usize = 3;
/// debug_check_board_coordinates!(x, y);     
/// debug_check_board_coordinates!(col: x);
/// ```    
#[macro_export]
macro_rules! debug_check_board_coordinates {
    ($x:expr, $y:expr) => {
        // Forces usize at compile time.
        let x: usize = $x;
        let y: usize = $y;
        debug_assert!(
            x < BOARD_WIDTH && y < BOARD_HEIGHT,
            "Illegal coordinates: x={}, y={} (valid: x < {}, y < {})",
            x,
            y,
            BOARD_WIDTH,
            BOARD_HEIGHT
        );
    };

    (col: $x:expr) => {
        let x: usize = $x;
        debug_assert!(
            x < BOARD_WIDTH,
            "Illegal column: {} (valid: col < {})",
            x,
            BOARD_WIDTH
        );
    };
}
