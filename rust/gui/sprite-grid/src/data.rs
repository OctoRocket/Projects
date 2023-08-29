type PixelGrid = Vec<Vec<RGBA>>;

type TileGrid = Vec<Vec<Tile>>;

#[derive(Debug, Clone, Copy)]
pub struct RGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Debug)]
pub struct Grid {
    pub row_count: u32, // Number of Tile rows
    pub column_count: u32, // Number of Tile columns
    pub resolution: u32, // Resolution of each sprite
    pub scale_amount: u32, // How much to scale up each sprite
    pub line_thickness: u32, // How thick the grid lines are
    pub line_color: RGBA, // Color of the grid lines
}

#[derive(Debug)]
struct Tile {
    content: PixelGrid,
    id: String,
}

#[derive(Debug)]
pub struct Coordinate {
    pub x: u32,
    pub y: u32,
}
