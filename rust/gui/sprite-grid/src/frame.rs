use crate::types::{
    Grid,
    Rgba,
    Coord,
    Tile,
    RgbaGrid,
};

pub trait RGBAGridBuilder {
    fn new_grid(
        length: usize,
    ) -> RgbaGrid;
}
impl RGBAGridBuilder for RgbaGrid {
    fn new_grid(
            length: usize,
        ) -> RgbaGrid {
        let mut grid = Self::with_capacity(length);
        for _ in 0..length {
            grid.push(Rgba::new(
                0,
                0,
                0,
                None,
            ));
        }

        grid
    }
}

pub trait Canvas {
    fn set_pixel(
        &mut self,
        coord: Coord,
        color: Rgba,
        grid: &Grid,
    );

    fn fill_frame(
        &mut self,
        color: Rgba,
    );

    fn clear_frame(
        &mut self,
    );
}

impl Canvas for RgbaGrid {
    fn set_pixel(&mut self, coord: Coord, color: Rgba, grid: &Grid) {
        let index = (coord.y * grid.side_length + coord.x) as usize;

        self[index] = color;
    }

    fn fill_frame(&mut self, color: Rgba) {
        for rgba in self {
            *rgba = color;
        }
    }

    fn clear_frame(&mut self) {
        for rgba in self {
            *rgba = Rgba::default();
        }
    }
}

pub trait GridBased {
    fn place_tile(
        &mut self,
        tile: &Tile,
        index: usize,
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
        color: Rgba,
    );

    fn tile_fill(
        &mut self,
        tile: &Tile,
        grid: &Grid
    );
}

impl GridBased for RgbaGrid {
    fn place_tile(&mut self, tile: &Tile, position_index: usize, grid: &Grid) {
        let starting_position = grid.starting_positions[position_index];
        let tile_width = grid.resolution * grid.scale_amount;
        
        dbg!(tile.content.len());
        dbg!((tile_width - 1) * tile_width + (tile_width - 1));

        for y in 0..(tile_width) {
            for x in 0..(tile_width) {
                let tile_index = (y * tile_width + x) as usize;

                self.set_pixel(Coord::new(
                    starting_position.x + x,
                    starting_position.y + y,
                ),
                tile.content[tile_index],
                grid);
            }
        }
    }

    fn tile_fill(&mut self, tile: &Tile, grid: &Grid) {
        for starting_position in 0..grid.starting_positions.len() {
            self.place_tile(tile, starting_position, grid);
        }
    }

    fn draw_grid_lines(&mut self, grid: &Grid, side_length: u32) {
        for column in 0..side_length {
            if column % (grid.resolution * grid.scale_amount + grid.line_thickness) < grid.line_thickness {
                for row in 0..side_length {
                    self.set_pixel(
                        Coord::new(column, row),
                        grid.line_color,
                        grid,
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
                        grid,
                    );
                }
            }
        }
    }

    // Plot the starting locations and the pixels to the east, south, and southeast
    fn plot_starting_locations(&mut self, grid: &Grid, color: Rgba) {
        for coord in &grid.starting_positions {
            self.set_pixel(*coord, color, grid);
            self.set_pixel(
                Coord::new(coord.x + 1, coord.y),
                color,
                grid,
            );
            self.set_pixel(
                Coord::new(coord.x, coord.y + 1),
                color,
                grid,
            );
            self.set_pixel(
                Coord::new(coord.x + 1, coord.y + 1),
                color,
                grid,
            );
        }
    }
}

// Convert an RGBAGrid into a [u8] for use with the pixels crate
pub trait FromRGBAGrid {
    fn copy_from_vec(&mut self, rgba_grid: &RgbaGrid);
}

impl FromRGBAGrid for [u8] {
    fn copy_from_vec(&mut self, rgba_grid: &RgbaGrid) {
        for i in 0..self.len() {
            if i % 4 == 0 {
                self[i] = rgba_grid[i / 4].red;
            } else if i % 4 == 1 {
                self[i] = rgba_grid[i / 4].green;
            } else if i % 4 == 2 {
                self[i] = rgba_grid[i / 4].blue;
            } else if i % 4 == 3 {
                self[i] = rgba_grid[i / 4].alpha;
            }
        }
    }
}

