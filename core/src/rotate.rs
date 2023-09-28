use bevy::prelude::*;

use crate::{WorldScale, grid::{Grid, GridPosition, GridOffset, TileSize}, StaticObject, DynamicObject, math::grid_to_world};


#[derive(Event, Debug, Clone)]
pub enum GridRotationEvent {
    Clockwise,
    CounterClockwise,
}

pub fn rotate_grid(
    mut rotation_event: EventReader<GridRotationEvent>,
    tilesize: Query<(&TileSize, &WorldScale), With<Grid>>,
    mut tiles: Query<
        (&mut GridPosition, &mut Transform),
        (With<StaticObject>, Without<DynamicObject>),
    >,
    mut dynamic_objects: Query<
        (&mut GridPosition, &mut Transform, Option<&GridOffset>),
        (With<DynamicObject>, Without<StaticObject>),
    >,
) {
    if let Ok((tilesize, scale)) = tilesize.get_single() {
        for rotation_event in rotation_event.iter() {
            for (old_grid_position, old_transform) in tiles.iter_mut() {
                rotate(
                    rotation_event,
                    old_grid_position,
                    old_transform,
                    *tilesize,
                    *scale,
                    GridOffset(Vec2::default()),
                );
            }

            for (old_grid_position, old_transform, offset) in dynamic_objects.iter_mut() {
                let offset = match offset {
                    Some(o) => *o,
                    None => GridOffset(Vec2::default()),
                };

                rotate(
                    rotation_event,
                    old_grid_position,
                    old_transform,
                    *tilesize,
                    *scale,
                    offset,
                );
            }
        }
    }
}

fn rotate(
    rotation_event: &GridRotationEvent,
    mut old_grid_position: Mut<GridPosition>,
    mut old_transform: Mut<Transform>,
    tilesize: TileSize,
    scale: WorldScale,
    offset: GridOffset,
) {
    let new_grid_position = match rotation_event {
        GridRotationEvent::Clockwise => old_grid_position.clone().rotate_c(13), // TODO: Remove hardcoded numbers
        GridRotationEvent::CounterClockwise => old_grid_position.clone().rotate_cc(13),
    };

    old_grid_position.x = new_grid_position.x;
    old_grid_position.y = new_grid_position.y;
    old_grid_position.layer = new_grid_position.layer;

    let mut world_pos: Vec3 = grid_to_world(
        Vec3::from(new_grid_position),
        tilesize.width() * scale.0,
        tilesize.height() * scale.0,
    );
    world_pos.x += offset.0.x;
    world_pos.y += offset.0.y;

    old_transform.translation = world_pos;
}
