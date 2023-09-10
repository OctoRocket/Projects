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
};
use frame::{
    Canvas,
    GridBased,
    RGBAGridBuilder,
    FromRGBAGrid,
};

fn main() -> Result<()> {
    // Get sprites from directory
    // let sprites = read_dir("sprites/")?
    //     .fuse()
    //     .map(|f| f.unwrap()
    //         .path()
    //         .to_string_lossy()
    //         .to_string()
    //     )
    //     .collect::<Vec<String>>();

    // Define the grid
    let grid = Grid::new(
        5,
        4,
        32,
        2,
        Rgba::white(),
    );

    dbg!(grid.side_length);
    dbg!(grid.side_length * grid.side_length, grid.side_length * grid.side_length * 4);

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
    
    // Preform sanity check
    render!(pixels, rgba_grid,
        rgba_grid.fill_frame(Rgba::new(100, 60, 255, None)) => sleep(Duration::from_secs(1)),
        rgba_grid.clear_frame(),
        rgba_grid.plot_starting_locations(&grid, Rgba::white()) => sleep(Duration::from_secs(1)),
        rgba_grid.clear_frame(),
        rgba_grid.draw_grid_lines(&grid, grid.side_length)
    );

    stdin().read_line(&mut String::new())?;

    Ok(())
}
