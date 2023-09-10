/// A type alias for a grid of RGBA values.
pub type RgbaGrid = Vec<Rgba>;

/// A type alias for a grid of tiles.
pub type TileGrid = Vec<Tile>;

/// A type alias for a grid of RGBA pixels.
pub type PixelGrid = Vec<Rgba>;

/// A type alias for a tile specific coordinates.
pub type TileCoord = Coord;

/// A struct representing an RGBA color.
#[derive(Debug, Clone, Copy)]
pub struct Rgba {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

/// A struct representing a grid of tiles.
#[derive(Debug)]
pub struct Grid {
    pub size: u32, // Number of Tile rows
    pub resolution: u32, // Resolution of each sprite
    pub scale_amount: u32, // How much to scale up each sprite
    pub line_thickness: u32, // How thick the grid lines are
    pub line_color: Rgba, // Color of the grid lines
    pub side_length: u32, // Length of the sides
    pub starting_positions: Vec<Coord>, // Where each tiles is started to be drawn
}

/// A struct representing a tile.
#[derive(Debug)]
pub struct Tile {
    pub content: PixelGrid,
    pub id: String,
    pub layer: u32,
}

/// A struct representing a coordinate.
#[derive(Debug, Clone, Copy)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}
