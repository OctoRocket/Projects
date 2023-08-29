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
use std::io::stdin;

fn main() -> Result<()> {
    // Define the grid
    let mut grid = Grid::new(
        5,
        10,
        8,
        16,
        2,
        RGBA::default(),
    );

    // Define the width and height of the window, surface, and grid
    let width = grid.column_count * grid.resolution * grid.scale_amount 
        + (grid.column_count + 1) * grid.line_thickness;
    let height = grid.row_count * grid.resolution * grid.scale_amount
        + (grid.row_count + 1) * grid.line_thickness;

    // Make the window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Sprite Grid")
        .with_inner_size(winit::dpi::LogicalSize::new(
            width,
            height,
        ))
        .build(&event_loop)?;
    
    let surface_texture = SurfaceTexture::new(width, height, &window);
    let mut pixels = Pixels::new(height, width, surface_texture)?;

    let mut frame = pixels.frame_mut();

    // Draw the grid outline
    grid.draw_grid_lines(&mut frame, width, height);

    // Display
    pixels.render()?;

    stdin().read_line(&mut String::new())?;

    Ok(())
}
