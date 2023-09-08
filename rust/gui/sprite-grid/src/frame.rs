use crate::data::{
    Grid,
    RGBA,
    Coord,
    Tile,
    TileCoord,
};

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
