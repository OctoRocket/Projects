#![deny(clippy::all)]
#![warn(
    clippy::pedantic,
    clippy::restriction,
    clippy::nursery,
)]

mod data;
mod misc;
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

use data::{
    Grid,
    RGBA,
};
use frame::{
    Canvas,
    GridFrame,
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

    let frame = pixels.frame_mut();

    dbg!(frame.len());
    dbg!((grid.side_length - 1) * grid.side_length * 4 + (grid.side_length - 1) * 4);

    // Preform sanity check
    frame.fill_frame(RGBA::new(100, 60, 255, None));
    pixels.render()?;
    let frame = pixels.frame_mut();
    sleep(Duration::from_secs(1));
    frame.clear_frame();
    pixels.render()?;
    let frame = pixels.frame_mut();

    // Plot the starting locations
    frame.plot_starting_locations(&grid, RGBA::default());
    pixels.render()?;
    let frame = pixels.frame_mut();
    sleep(Duration::from_secs(1));
    frame.clear_frame();
    pixels.render()?;
    let frame = pixels.frame_mut();

    // Draw the grid outline
    frame.draw_grid_lines(&grid, grid.side_length);

    // Render the frame
    pixels.render()?;

    stdin().read_line(&mut String::new())?;

    Ok(())
}
