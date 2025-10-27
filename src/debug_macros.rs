//! Here we collect a couple of custom debug macros.


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
        let x: u32 = $x;
        let y: u32 = $y;
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
        let x: u32 = $x;
        debug_assert!(
            x < BOARD_WIDTH,
            "Illegal column: {} (valid: col < {})",
            x,
            BOARD_WIDTH
        );
    };
}
