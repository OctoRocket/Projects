use crate::types::{
    Grid,
    Rgba,
    Coord,
    Tile,
};
use bmp::open;
use anyhow::Result;

/// Macro for rendering a sprite grid to a pixel buffer and displaying it.
///
/// # Arguments
///
/// * `$pixels` - A mutable reference to a `PixelBuffer` object.
/// * `$rgba_grid` - An expression that evaluates to a vector of RGBA color values.
/// * `$code` - Arbitrary code to be executed before rendering the sprite grid.
/// * `$post_render_task` - Optional code to be executed after rendering the sprite grid.
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
    /// Creates a new tile with the given sprite, identifier, and layer.
    ///
    /// # Arguments
    ///
    /// * `sprite` - A string representing the path to the sprite file.
    /// * `id` - A string representing the unique identifier of the tile.
    /// * `layer` - An unsigned 32-bit integer representing the layer of the tile.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new `Tile` if successful, or an error if the sprite file
    /// could not be opened or decoded.
    pub fn new<T>(
        sprite_path: T,
        grid: &Grid,
        layer: u32,
    ) -> Result<Self> where T: ToString {
        let sprite_path = sprite_path.to_string();

        dbg!(sprite_path.clone());
        let image = open(sprite_path)?;

        // Convert the buffer to a PixelGrid
        let mut content = Vec::new();
        for x in 0..grid.resolution {
            for y in 0..grid.resolution {
                let pixel = image.get_pixel(x, y);
                content.push(Rgba::new(pixel.r, pixel.b, pixel.g, None));
            }
        };
        
        dbg!(content.len());

        // Scale the tile
        let mut scaled_content = Vec::new();
        for pixel in content {
            for _ in 0..grid.scale_amount {
                scaled_content.push(pixel);
            }
        }
        scaled_content.clone().as_slice().chunks_exact(grid.resolution as usize).for_each(|chunk| {
            for _ in 0..grid.scale_amount {
                scaled_content.extend(chunk.clone());
            }
        });

        dbg!(scaled_content.len());

        Ok(Self {
            content: scaled_content,
            layer,
        })
    }
}
