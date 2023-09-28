use approx::ulps_eq;
use bevy::prelude::*;

pub fn grid_to_world(grid_pos: Vec3, tile_width: f32, tile_height: f32) -> Vec3 {
    let tile_width_half = tile_width / 2.0;
    let tile_height_half = tile_height / 2.0;

    let world_x = (grid_pos.x - grid_pos.y) * tile_width_half;
    let world_y = (grid_pos.x + grid_pos.y) * tile_height_half;

    Vec3::new(
        world_x,
        world_y + (tile_height * (grid_pos.z - 1.0).clamp(0.0, f32::MAX)),
        grid_pos.z,
    )
}

pub fn world_to_grid(world_pos: Vec3, tile_width: f32, tile_height: f32) -> Vec3 {
    let tile_width_half = tile_width / 2.0;
    let tile_height_half = tile_height / 2.0;

    let grid_x = (world_pos.x / tile_width_half + world_pos.y / tile_height_half) / 2.0;
    let grid_y = (world_pos.y / tile_height_half - (world_pos.x / tile_width_half)) / 2.0;

    Vec3::new(grid_x, grid_y, world_pos.z)
}

pub fn rotate_vector(v: Vec3, rotation: f32) -> Vec3 {
    Vec3::new(
        v.x * rotation.to_radians().cos() + v.y * rotation.to_radians().sin(),
        v.x * rotation.to_radians().sin() + v.y * rotation.to_radians().cos(),
        v.z,
    )
}

pub fn approx_eq_vec3(v1: Vec3, v2: Vec3) -> bool {
    ulps_eq!(v1.x, v2.x, epsilon = f32::EPSILON)
        && ulps_eq!(v1.y, v2.y, epsilon = f32::EPSILON)
        && ulps_eq!(v1.z, v2.z, epsilon = f32::EPSILON)
}

pub fn is_inside_tile(
    world_pos: Vec3,
    target_grid_pos: Vec3,
    tile_width: f32,
    tile_height: f32,
) -> bool {
    let world_grid_pos = world_to_grid(world_pos, tile_width, tile_height);
    let floored_abs_pos = world_grid_pos.floor();

    return floored_abs_pos.x >= 0.0
        && floored_abs_pos.y >= 0.0
        && target_grid_pos == floored_abs_pos;
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Vec3;

    use crate::math::approx_eq_vec3;
    use crate::math::grid_to_world;
    use crate::math::rotate_vector;
    use crate::math::is_inside_tile;
    use crate::math::world_to_grid;

    #[test]
    fn test_grid_to_world() {
        let grid_pos = Vec3::new(1.0, 0.0, 2.0);
        let target = Vec3::new(64.0, 96.0, 2.0);

        assert_eq!(grid_to_world(grid_pos, 128.0, 64.0), target);
    }

    #[test]
    fn test_grid_to_world_inside() {
        let grid_pos = Vec3::new(0.5, 0.0, 0.0);
        let target = Vec3::new(32.0, 16.0, 0.0);

        assert_eq!(grid_to_world(grid_pos, 128.0, 64.0), target);
    }

    #[test]
    fn test_world_to_grid() {
        let world_pos = Vec3::new(64.0, 32.0, 0.0);
        let target = Vec3::new(1.0, 0.0, 0.0);

        assert_eq!(world_to_grid(world_pos, 128.0, 64.0), target);
    }

    #[test]
    fn test_world_to_grid_inside() {
        let world_pos = Vec3::new(32.0, 16.0, 0.0);
        let target = Vec3::new(0.5, 0.0, 0.0);

        assert_eq!(world_to_grid(world_pos, 128.0, 64.0), target);
    }

    #[test]
    fn test_is_inside_tile() {
        let world_pos = Vec3::new(312.85934, 254.7338, 0.0);
        let target = Vec3::new(12.0, 3.0, 0.0);

        assert!(is_inside_tile(world_pos, target, 32.0 * 2.0, 16.0 * 2.0));
    }

    #[test]
    fn test_is_outside_tile() {
        let world_pos = Vec3::new(-2.2445679, -73.335556, 0.0);
        let target = Vec3::new(0.0, 0.0, 0.0);

        assert!(!is_inside_tile(world_pos, target, 32.0, 16.0));
    }

    #[test]
    fn test_approx_eq_vec3() {
        let v1 = Vec3::new(0.0, 0.0, 0.0);
        let v2 = Vec3::new(-4.371139e-8_f32, 0.0, 0.0);

        assert!(approx_eq_vec3(v1, v2));
    }

    #[test]
    fn test_rotate_vector() {
        let old_pos = Vec3::new(1.0, 0.0, 0.0);
        let new_pos = Vec3::new(0.0, 1.0, 0.0);
        let rotated = rotate_vector(old_pos, 90.0);

        assert!(approx_eq_vec3(rotated, new_pos));
    }
}
