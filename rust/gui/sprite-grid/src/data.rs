pub type TileGrid = Vec<Tile>;
pub type PixelGrid = Vec<RGBA>;
pub type TileCoord = Coord;

#[derive(Debug, Clone, Copy)]
pub struct RGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Debug)]
pub struct Grid {
    pub size: u32, // Number of Tile rows
    pub resolution: u32, // Resolution of each sprite
    pub scale_amount: u32, // How much to scale up each sprite
    pub line_thickness: u32, // How thick the grid lines are
    pub line_color: RGBA, // Color of the grid lines
    pub side_length: u32,
    pub starting_positions: Vec<Coord>,
}

#[derive(Debug)]
pub struct Tile {
    pub content: PixelGrid,
    pub id: String,
    pub layer: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}
