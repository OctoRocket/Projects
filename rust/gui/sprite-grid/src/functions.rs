use super::data::{
    Grid,
    RGBA,
    Coordinate,
};

impl Grid {
    pub fn new(
        row_count: u32,
        column_count: u32,
        resolution: u32,
        scale_amount: u32,
        line_thickness: u32,
        line_color: RGBA,
    ) -> Grid {
        Grid {
            row_count,
            column_count,
            resolution,
            scale_amount,
            line_thickness,
            line_color,
        }
    }

    pub fn draw_grid_lines(&mut self, frame: &mut [u8], width: u32, height: u32) {
        for row in 1..height {
            if row % (self.resolution * self.scale_amount + self.line_thickness) < self.line_thickness {
                for column in 1..width {
                    set_pixel(
                        Coordinate::new(column, row),
                        self.line_color,
                        frame,
                        width,
                    );
                }
            }
        }
        
        for column in 1..width {
            if column % (self.resolution * self.scale_amount + self.line_thickness) < self.line_thickness {
                for row in 1..height {
                    set_pixel(
                        Coordinate::new(column, row),
                        self.line_color,
                        frame,
                        width,
                    );
                }
            }
        }
    }
}

impl RGBA {
    pub fn new(
        red: u8,
        green: u8,
        blue: u8,
        alpha: Option<u8>,
    ) -> RGBA {
        let alpha = alpha.unwrap_or(u8::MAX);

        RGBA {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl Coordinate {
    pub fn new(
        x: u32,
        y: u32,
    ) -> Coordinate {
        Coordinate {
            x,
            y,
        }
    }
}

impl Default for RGBA {
    fn default() -> RGBA {
        RGBA {
            red: u8::MAX,
            green: u8::MAX,
            blue: u8::MAX,
            alpha: u8::MAX,
        }
    }
}

fn set_pixel(coord: Coordinate, color: RGBA, frame: &mut [u8], width: u32) {
    let index = (coord.y * width + coord.x) as usize * 4;

    frame[index] = color.red;
    frame[index + 1] = color.green;
    frame[index + 2] = color.blue;
    frame[index + 3] = color.alpha;
}

pub fn sanity_check(frame: &mut [u8], width: u32, height: u32) {
    for row in 0..width {
        for column in 0..height {
            set_pixel(
                Coordinate::new(column, row),
                RGBA::new(
                    u8::MAX,
                    u8::MAX,
                    u8::MAX,
                    None,
                ),
                frame,
                width,
            );
        }
    }
}
