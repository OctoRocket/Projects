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
    fs::read_dir,
};
use functions::{
    fill_frame,
    clear_frame,
    place_tile,
};

use crate::{functions::plot_starting_locations, data::{TileCoord, Tile}};

fn main() -> Result<()> {
    // Get sprites from directory
    let sprites = read_dir("sprites/")?
        .fuse()
        .map(|f| f.unwrap()
            .path()
            .to_string_lossy()
            .to_string()
        )
        .collect::<Vec<String>>();

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

    // Draw the sprites
    // The is the part you want to edit to change which sprites are drawn
    let tile = Tile::new(
        sprites[0].clone(),
        "grass".to_string(),
        0
    )?;
    
    for row in 0..grid.size {
        for col in 0..grid.size {
            let coord = TileCoord::new(col, row);
            place_tile(frame, &tile, coord, &grid)
        }
    }

    // Render the frame
    pixels.render()?;

    stdin().read_line(&mut String::new())?;

    Ok(())
}
