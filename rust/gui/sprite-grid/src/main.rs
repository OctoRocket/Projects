#![deny(clippy::all)]
#![warn(
    clippy::pedantic,
    clippy::restriction,
    clippy::nursery,
)]

mod data;
mod functions;

use pixels::{
    Pixels,
    SurfaceTexture,
};
use anyhow::Result;
use data::{
    Grid,
    RGBA,
};
use winit::{
    event_loop::EventLoop,
    window::WindowBuilder,
};
use std::{
    io::stdin,
    thread::sleep,
    time::Duration,
};
use functions::{
    fill_frame,
    clear_frame,
};

use crate::functions::plot_starting_locations;

fn main() -> Result<()> {
    // Define the grid
    let mut grid = Grid::new(
        5,
        4,
        32,
        2,
        RGBA::default(),
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

    let mut frame = pixels.frame_mut();

    dbg!(frame.len());
    dbg!((grid.side_length - 1) * grid.side_length * 4 + (grid.side_length - 1) * 4);

    // Preform sanity check
    fill_frame(&mut frame, RGBA::new(100, 60, 255, None));
    pixels.render()?;
    let mut frame = pixels.frame_mut();
    sleep(Duration::from_secs(1));
    clear_frame(&mut frame);
    pixels.render()?;
    let mut frame = pixels.frame_mut();

    // Plot the starting locations
    plot_starting_locations(&mut frame, &grid, RGBA::default());
    pixels.render()?;
    let mut frame = pixels.frame_mut();
    sleep(Duration::from_secs(1));
    clear_frame(&mut frame);
    pixels.render()?;
    let mut frame = pixels.frame_mut();

    // Draw the grid outline
    grid.draw_grid_lines(&mut frame, grid.side_length);
    pixels.render()?;

    stdin().read_line(&mut String::new())?;

    Ok(())
}
