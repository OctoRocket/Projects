use super::data::{
    Grid,
    RGBA,
    Coord,
    Tile,
    TileGrid,
    TileCoord,
};
use std::{
    path::PathBuf,
    fs::{
        File,
        read_dir,
    }
};
use png::Decoder;
use anyhow::Result;

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
                starting_positions.push(Coord::new(
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
                        Coord::new(column, row),
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
                        Coord::new(column, row),
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

impl Coord {
    pub fn new(
        x: u32,
        y: u32,
    ) -> Coord {
        Coord {
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

impl Tile {
    pub fn new(
        sprite: String,
        id: String,
        layer: u32,
    ) -> Result<Tile> {
        dbg!(sprite.clone());
        let mut reader = Decoder::new(File::open(sprite)?).read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        reader.next_frame(&mut buf)?;

        // Convert the buffer to a PixelGrid
        let mut content = Vec::new();
        for i in 0..buf.len() {
            if i % 4 == 0 {
                content.push(RGBA::new(
                    buf[i],
                    buf[i + 1],
                    buf[i + 2],
                    Some(buf[i + 3]),
                ));
            }
        };

        Ok(Tile {
            content,
            id,
            layer,
        })
    }
}

trait TileGridTrait {
    fn new_from_directory(
        directory: PathBuf,
    ) -> Result<TileGrid>;
}

impl TileGridTrait for TileGrid {
    fn new_from_directory(
        directory: PathBuf,
    ) -> Result<TileGrid> {
        let mut tiles = Vec::new();

        for entry in read_dir(directory)? {
            let path = entry?.path();
            let file_name = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let tile = Tile::new(
                path.to_string_lossy().to_string(),
                file_name,
                0,
            )?;

            tiles.push(tile);
        }

        Ok(tiles)
    }
}

fn set_pixel(coord: Coord, color: RGBA, frame: &mut [u8], pixel_size: u32) {
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
            Coord::new(coord.x + 1, coord.y),
            color,
            frame,
            grid.side_length,
        );
        set_pixel(
            Coord::new(coord.x, coord.y + 1),
            color,
            frame,
            grid.side_length,
        );
        set_pixel(
            Coord::new(coord.x + 1, coord.y + 1),
            color,
            frame,
            grid.side_length,
        );
    }
}

fn tile_coord_to_grid_coord(tile_coord: TileCoord, grid: &Grid) -> Coord {
    grid.starting_positions[(tile_coord.y * grid.size + tile_coord.x) as usize]
}

pub fn place_tile(frame: &mut [u8], tile: &Tile, coord: TileCoord, grid: &Grid) {
    let starting_coord = tile_coord_to_grid_coord(coord, grid);
    let wrap_coord = Coord::new(
        starting_coord.x + grid.resolution * grid.scale_amount,
        starting_coord.y + grid.resolution * grid.scale_amount,
    );
    
    // Draw to the frame
    for column in starting_coord.x..wrap_coord.x {
        for row in starting_coord.y..wrap_coord.y {
            let index = (row * grid.side_length * 4 + column * 4) as usize;
            let tile_index = ((row - starting_coord.y) * grid.resolution * 4 + (column - starting_coord.x) * 4) as usize;

            frame[index] = tile.content[tile_index].red;
            frame[index + 1] = tile.content[tile_index].green;
            frame[index + 2] = tile.content[tile_index].blue;
            frame[index + 3] = tile.content[tile_index].alpha;
        }
    }
}
