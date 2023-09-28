use bevy::prelude::*;

/// Marks an entity as part of a tilemap.
#[derive(Component, Copy, Clone, Debug)]
pub struct TilemapMarker;

/// The name of a tilemap, for debugging purposes.
#[derive(Component, Clone, Debug)]
pub struct TilemapName(String);

/// Ordering id for the tilemap layers.
#[derive(Component, Clone, Debug)]
pub struct TilemapOrderId(usize);

/// Bundle for creating tilemap entities.
#[derive(Bundle)]
pub struct TilemapBundle {
    _t: TilemapMarker,
    name: TilemapName,
    order_id: TilemapOrderId,
}

impl TilemapBundle {
    pub fn new(name: &str, id: usize) -> Self {
        Self {
            _t: TilemapMarker,
            name: TilemapName(String::from(name)),
            order_id: TilemapOrderId(id),
        }
    }
}