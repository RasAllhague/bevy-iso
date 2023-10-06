use std::path::{Path, PathBuf};

use bevy::reflect::{Reflect, TypeUuid};
use serde::{Deserialize, Serialize};

use super::tileset::TileSize;

#[derive(TypeUuid, Reflect, Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[uuid = "dd9b8ac0-170d-4ac5-a915-12fffd75df35"]
pub struct TilemapDefinition {
    name: String,
    tilesets: Vec<TilesetLink>,
    tile_size: TileSize,
    layers: Vec<LayerDefinition>,
}

#[derive(Serialize, Reflect, Deserialize, Debug, Clone, PartialEq)]
pub struct LayerDefinition {
    ordering_id: u32,
    tiles: Vec<Vec<TileIdentifier>>,
}

#[derive(Reflect, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TilesetLink {
    path: PathBuf,
    alias: char,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TilemapDefinitionBuilder {
    name: String,
    tilesets: Vec<TilesetLink>,
    tile_size: Option<TileSize>,
    layers: Vec<LayerDefinition>,
}

#[derive(Serialize, Reflect, Deserialize, Debug, Clone, PartialEq)]
pub struct TileIdentifier(String);

impl LayerDefinition {
    pub fn new(ordering_id: u32) -> Self {
        Self {
            ordering_id,
            tiles: Vec::new(),
        }
    }

    pub fn ordering_id(&self) -> u32 {
        self.ordering_id
    }

    pub fn tiles(&self) -> &[Vec<TileIdentifier>] {
        &self.tiles
    }
}

impl TilesetLink {
    pub fn new(path: &Path, alias: char) -> Self {
        TilesetLink {
            path: path.to_owned(),
            alias,
        }
    }

    pub fn alias(&self) -> char {
        self.alias
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl TileIdentifier {
    pub fn new(id: u32, alias: char) -> Self {
        Self(format!("{id}_{alias}"))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl TilemapDefinitionBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            tilesets: Vec::new(),
            tile_size: None,
            layers: Vec::new(),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_owned();
        self
    }

    pub fn with_tile_size(mut self, width: usize, height: usize) -> Self {
        self.tile_size = Some(TileSize::new(width, height));
        self
    }

    pub fn add_tileset(mut self, tileset_link: TilesetLink) -> Self {
        let index = self
            .tilesets
            .iter()
            .enumerate()
            .filter(|(_, tile_link)| tile_link.path == tileset_link.path)
            .map(|(index, _)| index)
            .next();

        if let Some(i) = index {
            self.tilesets[i] = tileset_link;
            return self;
        }

        self.tilesets.push(tileset_link);
        self
    }

    pub fn remove_tileset(mut self, path: &Path) -> Self {
        let index = self
            .tilesets
            .iter()
            .enumerate()
            .filter(|(_, tile_link)| tile_link.path == path)
            .map(|(index, _)| index)
            .next();

        if let Some(i) = index {
            self.tilesets.remove(i);
            return self;
        }

        self
    }

    pub fn add_layer(mut self, layer_def: LayerDefinition) -> Self {
        let index = self
            .layers
            .iter()
            .enumerate()
            .filter(|(_, layer)| layer.ordering_id == layer_def.ordering_id)
            .map(|(index, _)| index)
            .next();

        if let Some(i) = index {
            self.layers[i] = layer_def;
            return self;
        }

        self.layers.push(layer_def);
        self
    }

    pub fn remove_layer(mut self, ordering_id: u32) -> Self {
        let index = self
            .layers
            .iter()
            .enumerate()
            .filter(|(_, layer)| layer.ordering_id == ordering_id)
            .map(|(index, _)| index)
            .next();

        if let Some(i) = index {
            self.layers.remove(i);
            return self;
        }

        self
    }

    pub fn build(self) -> TilemapDefinition {
        let tile_size = match self.tile_size {
            Some(ts) => ts,
            None => TileSize::new(16, 16),
        };

        TilemapDefinition {
            name: self.name,
            tilesets: self.tilesets,
            tile_size,
            layers: self.layers,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::loading::tilemap::{TilemapDefinition, TilemapDefinitionBuilder, TilesetLink};
    use crate::loading::tileset::TileSize;
    use std::path::Path;

    #[test]
    fn test_add_layer() {
        let definition = TilemapDefinitionBuilder::new("testmap.json")
            .add_tileset(TilesetLink::new(Path::new("./testset.json"), 't'))
            .build();

        assert_eq!(
            TilemapDefinition {
                name: String::from("testmap.json"),
                tilesets: vec![TilesetLink::new(Path::new("./testset.json"), 't')],
                tile_size: TileSize::new(16, 16),
                layers: Vec::new(),
            },
            definition
        );
    }

    #[test]
    fn test_remove_layer() {
        let definition = TilemapDefinitionBuilder::new("testmap.json")
            .add_tileset(TilesetLink::new(Path::new("./testset.json"), 't'))
            .remove_tileset(Path::new("./testset.json"))
            .build();

        assert_eq!(
            TilemapDefinition {
                name: String::from("testmap.json"),
                tilesets: Vec::new(),
                tile_size: TileSize::new(16, 16),
                layers: Vec::new(),
            },
            definition
        );
    }

    #[test]
    fn test_with_tilesize() {
        let definition = TilemapDefinitionBuilder::new("testmap.json")
            .with_tile_size(32, 32)
            .build();

        assert_eq!(
            TilemapDefinition {
                name: String::from("testmap.json"),
                tilesets: Vec::new(),
                tile_size: TileSize::new(32, 32),
                layers: Vec::new(),
            },
            definition
        );
    }
}
