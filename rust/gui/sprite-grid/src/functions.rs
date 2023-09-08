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

trait TileGridBuilder {
    fn new_from_directory(
        directory: PathBuf,
    ) -> Result<TileGrid>;
}

impl TileGridBuilder for TileGrid {
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

pub trait Canvas {
    fn set_pixel(
        &mut self,
        coord: Coord,
        color: RGBA,
        pixel_size: u32,
    );

    fn fill_frame(
        &mut self,
        color: RGBA,
    );

    fn clear_frame(
        &mut self,
    );
}

impl Canvas for [u8] {
    fn set_pixel(&mut self, coord: Coord, color: RGBA, pixel_size: u32) {
        let index = (coord.y * pixel_size * 4 + coord.x * 4) as usize;
        // dbg!(coord);

        self[index] = color.red;
        self[index + 1] = color.green;
        self[index + 2] = color.blue;
        self[index + 3] = color.alpha;
    }

    fn fill_frame(&mut self, color: RGBA) {
        for i in 0..self.len() {
            if i % 4 == 0 {
                self[i] = color.red;
            } else if i % 4 == 1 {
                self[i] = color.green;
            } else if i % 4 == 2 {
                self[i] = color.blue;
            } else if i % 4 == 3 {
                self[i] = color.alpha;
            }
        }
    }

    fn clear_frame(&mut self) {
        for i in 0..self.len() {
            self[i] = 0;
        }
    }
}

pub trait GridFrame {
    fn place_tile(
        &mut self,
        tile: &Tile,
        coord: TileCoord,
        grid: &Grid,
    );

    fn draw_grid_lines(
        &mut self,
        grid: &Grid,
        side_length: u32,
    );

    fn plot_starting_locations(
        &mut self,
        grid: &Grid,
        color: RGBA,
    );
}

impl GridFrame for [u8] {
    fn place_tile(&mut self, tile: &Tile, coord: TileCoord, grid: &Grid) {
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
    
                self[index] = tile.content[tile_index].red;
                self[index + 1] = tile.content[tile_index].green;
                self[index + 2] = tile.content[tile_index].blue;
                self[index + 3] = tile.content[tile_index].alpha;
            }
        }
    }

    fn draw_grid_lines(&mut self, grid: &Grid, side_length: u32) {
        for column in 0..side_length {
            if column % (grid.resolution * grid.scale_amount + grid.line_thickness) < grid.line_thickness {
                for row in 0..side_length {
                    self.set_pixel(
                        Coord::new(column, row),
                        grid.line_color,
                        side_length,
                    );
                }
            }
        }

        for row in 0..side_length {
            if row % (grid.resolution * grid.scale_amount + grid.line_thickness) < grid.line_thickness {
                for column in 0..side_length {
                    self.set_pixel(
                        Coord::new(column, row),
                        grid.line_color,
                        side_length,
                    );
                }
            }
        }
    }

    // Plot the starting locations and the pixels to the east, south, and southeast
    fn plot_starting_locations(&mut self, grid: &Grid, color: RGBA) {
        for coord in &grid.starting_positions {
            self.set_pixel(*coord, color, grid.side_length);
            self.set_pixel(
                Coord::new(coord.x + 1, coord.y),
                color,
                grid.side_length,
            );
            self.set_pixel(
                Coord::new(coord.x, coord.y + 1),
                color,
                grid.side_length,
            );
            self.set_pixel(
                Coord::new(coord.x + 1, coord.y + 1),
                color,
                grid.side_length,
            );
        }
    }
}

fn tile_coord_to_grid_coord(tile_coord: TileCoord, grid: &Grid) -> Coord {
    grid.starting_positions[(tile_coord.y * grid.size + tile_coord.x) as usize]
}
