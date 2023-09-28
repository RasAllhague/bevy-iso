
use bevy::prelude::*;

pub mod math;
pub mod tilemap;
pub mod grid;
pub mod tile;
pub mod ordering;
pub mod loading;
pub mod rotate;
pub mod spawning;
pub mod plugins;

/// Multiplier for calculation in case the world should be bigger or smaller.
#[derive(Component, Debug, Copy, Clone)]
pub struct WorldScale(pub f32);

/// Marker for objects that can move in the scene.
#[derive(Component, Debug, Copy, Clone)]
pub struct DynamicObject;

/// Marks an entity as a static tile. Z usualy will only update
#[derive(Component, Debug, Copy, Clone)]
pub struct StaticObject;