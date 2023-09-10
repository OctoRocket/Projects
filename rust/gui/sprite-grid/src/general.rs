use crate::types::{
    Grid,
    Rgba,
    Coord,
    Tile,
    TileGrid,
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

#[macro_export]
macro_rules! render {
    ($pixels:expr, $rgba_grid:expr, $($code:expr$(=> $post_render_task:expr)?),*) => {
        $(
            let frame = $pixels.frame_mut();
            $code;
            frame.copy_from_vec(&$rgba_grid);
            $pixels.render()?;
            $($post_render_task;)?
        )*
    };
}

impl Grid {
    /// Creates a new grid with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of sprites in each row and column of the grid.
    /// * `resolution` - The resolution of each sprite in pixels.
    /// * `scale_amount` - The amount to scale each sprite by.
    /// * `line_thickness` - The thickness of the lines separating the sprites.
    /// * `line_color` - The color of the lines separating the sprites.
    ///
    /// # Returns
    ///
    /// A new `Grid` instance.
    pub fn new(
        size: u32,
        resolution: u32,
        scale_amount: u32,
        line_thickness: u32,
        line_color: Rgba,
    ) -> Self {
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

        Self {
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

impl Rgba {
    pub fn new(
        red: u8,
        green: u8,
        blue: u8,
        alpha: Option<u8>,
    ) -> Self {
        let alpha = alpha.unwrap_or(u8::MAX);

        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub const fn white() -> Self {
        Self {
            red: u8::MAX,
            green: u8::MAX,
            blue: u8::MAX,
            alpha: u8::MAX,
        }
    }
}

impl Default for Rgba {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: u8::MAX,
        }
    }
}

impl Coord {
    pub const fn new(
        x: u32,
        y: u32,
    ) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl Tile {
    pub fn new(
        sprite: String,
        id: String,
        layer: u32,
    ) -> Result<Self> {
        dbg!(sprite.clone());
        let mut reader = Decoder::new(File::open(sprite)?).read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        reader.next_frame(&mut buf)?;

        // Convert the buffer to a PixelGrid
        let mut content = Vec::new();
        for i in 0..buf.len() {
            if i % 4 == 0 {
                content.push(Rgba::new(
                    buf[i],
                    buf[i + 1],
                    buf[i + 2],
                    Some(buf[i + 3]),
                ));
            }
        };

        Ok(Self {
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
        let mut tiles = Self::new();

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
