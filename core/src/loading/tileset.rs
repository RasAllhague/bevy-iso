use bevy::reflect::Reflect;
use itertools::Itertools;
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TilesetDefinition {
    name: String,
    tile_size: TileSize,
    source: SourceDefinition,
    tiles: Vec<TileDefinition>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnimatedTileDefBuilder {
    id: u32,
    intervals: f32,
    positions: Vec<TilePosition>,
}

#[derive(Reflect, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Default)]
pub struct TileSize {
    width: usize,
    height: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SourceDefinition {
    path: PathBuf,
    dimensions: ImageDimensions,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct ImageDimensions {
    width: usize,
    height: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TileDefinition {
    Standard {
        id: u32,
        x: usize,
        y: usize,
    },
    Animated {
        id: u32,
        positions: Vec<TilePosition>,
        interval_per_sec: f32,
    },
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct TilePosition {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TilesetDefinitionBuilder {
    source: SourceDefinition,
    tile_size: Option<TileSize>,
    name: Option<String>,
    tiles: Vec<TileDefinition>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidName,
    DublicatedTileIds(Vec<(u32, u32)>),
    DublicatedTilePositions(Vec<((usize, usize), Vec<u32>)>),
    TileOutOfBounds(u32),
}

impl TilePosition {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl ImageDimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self { height, width }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

impl SourceDefinition {
    pub fn new(path: &Path, dimensions: ImageDimensions) -> Self {
        Self {
            path: path.to_owned(),
            dimensions,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn dimensions(&self) -> ImageDimensions {
        self.dimensions
    }
}

impl TileSize {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl TileDefinition {
    pub fn new_standard(id: u32, x: usize, y: usize) -> Self {
        Self::Standard { id, x, y }
    }

    pub fn new_animated(id: u32, interval: f32) -> Self {
        Self::Animated {
            id: id,
            interval_per_sec: interval,
            positions: Vec::new(),
        }
    }

    pub fn id(&self) -> u32 {
        match self {
            Self::Standard { id, x: _, y: _ } => *id,
            Self::Animated {
                id,
                interval_per_sec: _,
                positions: _,
            } => *id,
        }
    }
}

impl TilesetDefinitionBuilder {
    pub fn new(source: SourceDefinition) -> Self {
        Self {
            name: None,
            source,
            tile_size: None,
            tiles: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn with_tile_size(mut self, width: usize, height: usize) -> Self {
        self.tile_size = Some(TileSize::new(width, height));
        self
    }

    pub fn with_source(mut self, source: SourceDefinition) -> Self {
        self.source = source;
        self
    }

    pub fn add_tile(mut self, tile: TileDefinition) -> Self {
        for i in 0..self.tiles.len() {
            if self.tiles[i].id() == tile.id() {
                self.tiles[i] = tile;
                return self;
            }
        }

        self.tiles.push(tile);
        self
    }

    pub fn remove_tile(mut self, id: u32) -> Self {
        for i in 0..self.tiles.len() {
            if self.tiles[i].id() == id {
                self.tiles.remove(i);
                return self;
            }
        }

        self
    }

    pub fn build(self) -> Result<TilesetDefinition, Error> {
        let dublicated_ids = Self::get_dublicated_ids(&self.tiles);

        if dublicated_ids.len() != 0 {
            return Err(Error::DublicatedTileIds(dublicated_ids));
        }

        let dublicated_positions = Self::get_dublicated_positions(&self.tiles);

        if dublicated_positions.len() != 0 {
            return Err(Error::DublicatedTilePositions(dublicated_positions));
        }

        // if !Self::tile_is_in_bounds() {
        // TODO: DO THIS!!!
        // }

        let name = match self.name {
            Some(n) => n,
            None => Path::new(&self.source.path)
                .file_name()
                .ok_or(Error::InvalidName)?
                .to_str()
                .ok_or(Error::InvalidName)?
                .to_owned(),
        };

        let tile_size = match self.tile_size {
            Some(t) => t,
            None => TileSize::new(16, 16),
        };

        Ok(TilesetDefinition {
            name,
            source: self.source,
            tile_size,
            tiles: self.tiles.clone(),
        })
    }

    fn get_dublicated_ids(tiles: &[TileDefinition]) -> Vec<(u32, u32)> {
        tiles
            .iter()
            .group_by(|tile| tile.id())
            .into_iter()
            .map(|(key, group)| (key, group.count() as u32))
            .filter(|(id, count)| *count >= 2)
            .collect()
    }

    fn get_dublicated_positions(tiles: &[TileDefinition]) -> Vec<((usize, usize), Vec<u32>)> {
        tiles
            .iter()
            .filter(|tile| tile.is_standard())
            .map(|tile| match tile {
                TileDefinition::Standard { id, x, y } => Some((*id, *x, *y)),
                TileDefinition::Animated {
                    id: _,
                    positions: _,
                    interval_per_sec: _,
                } => None,
            })
            .flatten()
            .group_by(|(_, x, y)| (*x, *y))
            .into_iter()
            .map(|((x, y), group)| ((x, y), group.map(|(id, _, _)| id).collect::<Vec<u32>>()))
            .collect()
    }

    fn tile_is_in_bounds(
        tile: TileDefinition,
        tile_size: TileSize,
        image_dimensions: ImageDimensions,
    ) -> bool {
        todo!();
    }
}

impl TileDefinition {
    pub fn is_standard(&self) -> bool {
        match self {
            TileDefinition::Standard { id: _, x: _, y: _ } => true,
            TileDefinition::Animated {
                id: _,
                positions: _,
                interval_per_sec: _,
            } => false,
        }
    }

    pub fn is_animated(&self) -> bool {
        match self {
            TileDefinition::Standard { id: _, x: _, y: _ } => false,
            TileDefinition::Animated {
                id: _,
                positions: _,
                interval_per_sec: _,
            } => true,
        }
    }
}

impl AnimatedTileDefBuilder {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            intervals: 0.5,
            positions: Vec::new(),
        }
    }

    pub fn with_interval(mut self, interval: f32) -> Self {
        self.intervals = interval;
        self
    }

    pub fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn add_position(mut self, position: TilePosition) -> Self {
        self.positions.push(position);
        self
    }

    pub fn remove_position(mut self, position: TilePosition) -> Self {
        for i in 0..self.positions.len() {
            let index_pos = self.positions[i];

            if index_pos.x == position.x && index_pos.y == index_pos.y {
                self.positions.remove(i);
                return self;
            }
        }

        return self;
    }

    pub fn clear_positions(mut self) -> Self {
        self.positions.clear();
        self
    }

    pub fn build(self) -> TileDefinition {
        TileDefinition::Animated {
            id: self.id,
            positions: self.positions,
            interval_per_sec: self.intervals,
        }
    }
}