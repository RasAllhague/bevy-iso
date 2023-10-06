use bevy::prelude::*;

use crate::{
    ordering::{order_static_tile_z, reorder_on_rotation, update_dynamic_object_z},
    rotate::{rotate_grid, GridRotationEvent}, loading::{loader::TilemapAssetLoader, tilemap::TilemapDefinition},
};

pub struct IsometricTilemapPlugin;

impl Plugin for IsometricTilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GridRotationEvent>()
            .add_asset::<TilemapDefinition>()
            .init_asset_loader::<TilemapAssetLoader>()
            .add_systems(Update,(
                order_static_tile_z.before(reorder_on_rotation),
                update_dynamic_object_z,
                rotate_grid.before(reorder_on_rotation),
                reorder_on_rotation.after(rotate_grid),
            ));
    }
}