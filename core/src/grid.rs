use bevy::prelude::*;

use crate::loading::TilemapFile;

/// Marker for grid entities.
#[derive(Component, Copy, Clone, Debug)]
pub struct GridMarker;

/// Contains grid data.
#[derive(Component, Clone, Debug)]
pub struct Grid {
    pub tilemap_handle: Handle<TilemapFile>,
    pub texture_atlas_handle: Option<Handle<TextureAtlas>>,
}

/// Identifies a position in the grid.
#[derive(Default, Component, Clone, PartialEq, Copy, Debug,)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
    pub layer: usize,
}

/// Bundle for creating grid entities.
#[derive(Bundle)]
pub struct GridBundle {
    _g: GridMarker,
    grid: Grid,
}

/// Offset a grid from the center of the world.
#[derive(Component, Copy, Clone)]
pub struct GridOffset(pub Vec2);

/// The size of a single tile in the tilemap.
#[derive(Component, Copy, Clone, Debug)]
pub struct TileSize {
    height: f32,
    width: f32,
}

impl GridBundle {
    pub fn new(grid: Grid) -> Self {
        Self {
            _g: GridMarker,
            grid,
        }
    }
}

impl GridPosition {
    pub fn new(x: usize, y: usize, layer: usize) -> Self {
        Self { x, y, layer }
    }

    /// Rotates the grid position clockwise
    pub fn rotate_c(self, n: usize) -> Self {
        Self {
            x: self.y,
            y: n - self.x - 1,
            layer: self.layer,
        }
    }

    /// rotates the grid position counterclockwise
    pub fn rotate_cc(self, n: usize) -> Self {
        Self {
            x: n - self.y - 1,
            y: self.x,
            layer: self.layer,
        }
    }
}

impl From<Vec3> for GridPosition {
    fn from(value: Vec3) -> Self {
        Self {
            x: value.x as usize,
            y: value.y as usize,
            layer: value.z as usize,
        }
    }
}

impl From<GridPosition> for Vec3 {
    fn from(value: GridPosition) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
            z: value.layer as f32,
        }
    }
}

impl TileSize {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn width(&self) -> f32 {
        self.width
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::GridPosition;

    #[test]
    fn test_rotate_clockwise() {
        let p1 = GridPosition::new(0, 0, 0);
        let p2 = GridPosition::new(2, 1, 0);

        let r1 = p1.rotate_c(4);
        let r2 = p2.rotate_c(4);

        assert_eq!(GridPosition::new(0, 3, 0), r1);
        assert_eq!(GridPosition::new(1, 1, 0), r2);
    }

    #[test]
    fn test_rotate_counter_clockwise() {
        let p1 = GridPosition::new(0, 0, 0);
        let p2 = GridPosition::new(2, 1, 0);

        let r1 = p1.rotate_cc(4);
        let r2 = p2.rotate_cc(4);

        assert_eq!(GridPosition::new(3, 0, 0), r1);
        assert_eq!(GridPosition::new(2, 2, 0), r2);
    }
}