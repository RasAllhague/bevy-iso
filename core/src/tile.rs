use bevy::prelude::*;

use crate::{StaticObject, ordering::ZOffset};

use super::grid::GridPosition;

/// Marker for tile entities.
#[derive(Component)]
pub struct TileMarker;

/// Id for identifieng tiles.
#[derive(Component)]
pub struct TileId(u32);

/// Bundle for creating tile entities.
#[derive(Bundle)]
pub struct TileBundle {
    _m: TileMarker,
    _s: StaticObject,
    id: TileId,
    pos: GridPosition,
    z_offset: ZOffset,
    // TODO: Set bundle
    sprite_sheet: SpriteSheetBundle,
}

impl TileId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn id(&self) -> u32 {
        self.0
    }
}

impl TileBundle {
    pub fn new(
        id: TileId,
        pos: GridPosition,
        z_offset: ZOffset,
        sprite_sheet: SpriteSheetBundle,
    ) -> Self {
        Self {
            _m: TileMarker,
            _s: StaticObject,
            id,
            pos,
            z_offset,
            sprite_sheet,
        }
    }
}