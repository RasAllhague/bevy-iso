use bevy::prelude::*;

use crate::{rotate::GridRotationEvent, StaticObject, DynamicObject};

/// Offset for layering dynamic objects and static tiles.
#[derive(Component, Debug, Copy, Clone)]
pub struct ZOffset(pub f32);

/// Sorts the static tiles basedon their z offeset and their y position substracted from their z.
pub fn order_static_tile_z(
    mut static_tiles: Query<(&mut Transform, &ZOffset), (Added<StaticObject>, With<StaticObject>)>,
) {
    for (mut object_transform, object_offset) in static_tiles.iter_mut() {
        object_transform.translation.z =
            calculate_z_order(object_transform.translation, object_offset);
    }
}

pub fn reorder_on_rotation(
    mut rotation_event: EventReader<GridRotationEvent>,
    mut static_tiles: Query<(&mut Transform, &ZOffset), With<StaticObject>>,
) {
    for _ in rotation_event.iter() {
        for (mut object_transform, object_offset) in static_tiles.iter_mut() {
            debug!("Old Z ordering: {}", object_transform.translation.z);

            object_transform.translation.z =
                calculate_z_order(object_transform.translation, object_offset);

            debug!("New Z ordering: {}", object_transform.translation.z);
        }
    }
}

/// Sorts the entities z position to change what on top via a zoffset to do the basic layer and then substracting y from z.
pub fn update_dynamic_object_z(
    mut dynamic_objects: Query<(&mut Transform, &ZOffset), With<DynamicObject>>,
) {
    for (mut object_transform, object_offset) in dynamic_objects.iter_mut() {
        object_transform.translation.z =
            calculate_z_order(object_transform.translation, object_offset);
    }
}

fn calculate_z_order(orthogonal_position: Vec3, z_offset: &ZOffset) -> f32 {
    return z_offset.0 - orthogonal_position.y / 100.0;
}
