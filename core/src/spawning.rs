use bevy::prelude::*;

use crate::{WorldScale, grid::{Grid, TileSize, GridPosition}, loading::TilemapFile, tile::{TileBundle, TileId}, ordering::ZOffset, tilemap::TilemapBundle, math::grid_to_world};

pub fn spawn_tilemap(
    mut commands: Commands,
    mut new_grids: Query<(Entity, &WorldScale, &mut Grid), Added<Grid>>,
    asset_server: Res<AssetServer>,
    tilemaps: Res<Assets<TilemapFile>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (grid_entity, scale, mut grid) in new_grids.iter_mut() {
        if let Some(tilemap_file) = tilemaps.get(&grid.tilemap_handle) {
            let tilesize =
                TileSize::new(tilemap_file.tile_width(), tilemap_file.tile_height() / 2.0);
            commands.entity(grid_entity).insert((
                tilesize,
                Name::new(format!("Grid - {}", tilemap_file.name())),
            ));

            let texture_handle = asset_server.load(tilemap_file.source_image());
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(tilemap_file.tile_width(), tilemap_file.tile_height()),
                tilemap_file.columns(),
                tilemap_file.rows(),
                None,
                None,
            );

            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            grid.texture_atlas_handle = Some(texture_atlas_handle.clone());

            let mut layers = tilemap_file.layers();
            layers.sort_by_key(|x| x.order_id());

            for layer in layers {
                let tiles = layer.tiles();
                let tilemap_height = tiles.len();
                let tilemap_width = tiles[0].len();

                for y in 0..tilemap_height {
                    for x in 0..tilemap_width {
                        if tiles[y][x] != -1 {
                            let pos: Vec3 = Vec3::new(x as f32, y as f32, layer.order_id() as f32);

                            let mut transform = Transform::default();
                            transform.translation = grid_to_world(
                                pos,
                                tilesize.width() * scale.0,
                                tilesize.height() * scale.0,
                            );
                            transform.scale = Vec3::new(scale.0, scale.0, 0.0);

                            commands.spawn((
                                TileBundle::new(
                                    TileId::new(tiles[y][x] as u32),
                                    GridPosition::new(x, y, layer.order_id()),
                                    ZOffset(layer.order_id() as f32 * 100.0),
                                    SpriteSheetBundle {
                                        texture_atlas: texture_atlas_handle.clone(),
                                        transform: transform,
                                        sprite: TextureAtlasSprite::new(tiles[y][x] as usize),
                                        ..default()
                                    },
                                ),
                                Name::new(format!("Tile ({},{},{})", x, y, layer.order_id())),
                            ));
                        }
                    }
                }

                commands.spawn((
                    TilemapBundle::new(&layer.name(), layer.order_id()),
                    Name::new(format!("Tilemap - {}", layer.name())),
                ));
            }
        }
    }
}
