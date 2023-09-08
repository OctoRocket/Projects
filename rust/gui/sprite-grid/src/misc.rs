use super::data::{
    Grid,
    RGBA,
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

impl Grid {
    pub fn new(
        size: u32,
        resolution: u32,
        scale_amount: u32,
        line_thickness: u32,
        line_color: RGBA,
    ) -> Grid {
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

        Grid {
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

impl RGBA {
    pub fn new(
        red: u8,
        green: u8,
        blue: u8,
        alpha: Option<u8>,
    ) -> RGBA {
        let alpha = alpha.unwrap_or(u8::MAX);

        RGBA {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl Coord {
    pub fn new(
        x: u32,
        y: u32,
    ) -> Coord {
        Coord {
            x,
            y,
        }
    }
}

impl Default for RGBA {
    fn default() -> RGBA {
        RGBA {
            red: u8::MAX,
            green: u8::MAX,
            blue: u8::MAX,
            alpha: u8::MAX,
        }
    }
}

impl Tile {
    pub fn new(
        sprite: String,
        id: String,
        layer: u32,
    ) -> Result<Tile> {
        dbg!(sprite.clone());
        let mut reader = Decoder::new(File::open(sprite)?).read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        reader.next_frame(&mut buf)?;

        // Convert the buffer to a PixelGrid
        let mut content = Vec::new();
        for i in 0..buf.len() {
            if i % 4 == 0 {
                content.push(RGBA::new(
                    buf[i],
                    buf[i + 1],
                    buf[i + 2],
                    Some(buf[i + 3]),
                ));
            }
        };

        Ok(Tile {
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
        let mut tiles = Vec::new();

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
