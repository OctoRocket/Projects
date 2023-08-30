use super::data::{
    Grid,
    RGBA,
    Coordinate,
};

impl Grid {
    pub fn new(
        size: u32,
        resolution: u32,
        scale_amount: u32,
        line_thickness: u32,
        line_color: RGBA,
    ) -> Grid {
        let side_length = size * resolution * scale_amount 
        + (size + 1) * line_thickness;

        let mut starting_positions = Vec::new();
        for column in 0..size {
            for row in 0..size {
                starting_positions.push(Coordinate::new(
                    column * (resolution * scale_amount + line_thickness) + line_thickness,
                    row * (resolution * scale_amount + line_thickness) + line_thickness,
                ));
            }
        }

        Grid {
            size,
            resolution,
            scale_amount,
            line_thickness,
            line_color,
            side_length,
            starting_positions,
        }
    }

    pub fn draw_grid_lines(&mut self, frame: &mut [u8], side_length: u32) {
        for column in 0..side_length {
            if column % (self.resolution * self.scale_amount + self.line_thickness) < self.line_thickness {
                for row in 0..side_length {
                    set_pixel(
                        Coordinate::new(column, row),
                        self.line_color,
                        frame,
                        side_length,
                    );
                }
            }
        }

        for row in 0..side_length {
            if row % (self.resolution * self.scale_amount + self.line_thickness) < self.line_thickness {
                for column in 0..side_length {
                    set_pixel(
                        Coordinate::new(column, row),
                        self.line_color,
                        frame,
                        side_length,
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

fn set_pixel(coord: Coordinate, color: RGBA, frame: &mut [u8], pixel_size: u32) {
    let index = (coord.y * pixel_size * 4 + coord.x * 4) as usize;
    // dbg!(coord);

    frame[index] = color.red;
    frame[index + 1] = color.green;
    frame[index + 2] = color.blue;
    frame[index + 3] = color.alpha;
}

pub fn fill_frame(frame: &mut [u8], color: RGBA) {
    for i in 0..frame.len() {
        if i % 4 == 0 {
            frame[i] = color.red;
        } else if i % 4 == 1 {
            frame[i] = color.green;
        } else if i % 4 == 2 {
            frame[i] = color.blue;
        } else if i % 4 == 3 {
            frame[i] = color.alpha;
        }
    }
}

pub fn clear_frame(frame: &mut [u8]) {
    for i in 0..frame.len() {
        frame[i] = u8::MIN;
    }
}

// Plot the starting locations and the pixels to the east, south, and southeast
pub fn plot_starting_locations(frame: &mut [u8], grid: &Grid, color: RGBA) {
    for coord in &grid.starting_positions {
        set_pixel(*coord, color, frame, grid.side_length);
        set_pixel(
            Coordinate::new(coord.x + 1, coord.y),
            color,
            frame,
            grid.side_length,
        );
        set_pixel(
            Coordinate::new(coord.x, coord.y + 1),
            color,
            frame,
            grid.side_length,
        );
        set_pixel(
            Coordinate::new(coord.x + 1, coord.y + 1),
            color,
            frame,
            grid.side_length,
        );
    }
}
