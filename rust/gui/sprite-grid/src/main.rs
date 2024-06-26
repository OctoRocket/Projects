#![deny(clippy::all)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
)]

mod types;
mod general;
mod frame;

use pixels::{
    Pixels,
    SurfaceTexture,
};
use anyhow::Result;
use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};
use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
    // fs::read_dir,
};

use types::{
    Grid,
    Rgba,
    RgbaGrid,
    Tile,
};
use frame::{
    Canvas,
    GridBased,
    RGBAGridBuilder,
    FromRGBAGrid,
};

fn main() -> Result<()> {
    // Define the grid
    let grid = Grid::new(
        5,
        8,
        12,
        4,
        Rgba::default(),
    );

    // Make the window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Sprite Grid")
        .with_inner_size(winit::dpi::LogicalSize::new(
            grid.side_length,
            grid.side_length,
        ))
        .with_resizable(false)
        .build(&event_loop)?;
    
    let surface_texture = SurfaceTexture::new(grid.side_length, grid.side_length, &window);
    let mut pixels = Pixels::new(grid.side_length, grid.side_length, surface_texture)?;

    let mut rgba_grid = RgbaGrid::new_grid(pixels.frame().len());
    
    // Preform basic setup
    render!(pixels, rgba_grid,
        // Preform sanity check
        rgba_grid.fill_frame(Rgba::new(100, 60, 255, None)) => sleep(Duration::from_millis(250)),
        rgba_grid.clear_frame(),
        // plot tile draw-from locations
        rgba_grid.plot_starting_locations(&grid, Rgba::white()) => sleep(Duration::from_millis(250)),
        rgba_grid.clear_frame(),
        // draw the grid lines
        rgba_grid.draw_grid_lines(&grid, grid.side_length) => sleep(Duration::from_millis(250))
    );

    // Make a grass tile and then fill the grid with it
    let grass_tile = Tile::new(
        &"sprites/grass.bmp",
        &grid,
        0,
    )?;

    render!(pixels, rgba_grid,
        rgba_grid.tile_fill(&grass_tile, &grid)
    );

    stdin().read_line(&mut String::new())?;

    Ok(())
}
