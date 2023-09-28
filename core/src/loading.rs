use anyhow::Result;
use bevy::{
    asset::{AssetLoader, LoadedAsset},
    reflect::{TypeUuid, Reflect},
};
use serde::{Serialize, Deserialize};
use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Write,
};

/// Asset for loading and creating tilemaps from files.
#[derive(TypeUuid)]
#[uuid = "dd9b8ac0-170d-4ac5-a915-12fffd75df35"]
#[derive(Reflect, Serialize, Deserialize, Default)]
pub struct TilemapFile {
    name: String,
    source_image: String,
    columns: usize,
    rows: usize,
    tile_height: f32,
    tile_width: f32,
    layers: Vec<LayerDefinition>,
}

/// Single layer definition of the tilemap.
#[derive(Reflect, Serialize, Deserialize, Default, Clone)]
pub struct LayerDefinition {
    order_id: usize,
    name: String,
    tiles: Vec<Vec<i32>>,
}

/// Asset loader for tilemap assets
#[derive(Default)]
pub struct TilemapAssetLoader;

impl AssetLoader for TilemapAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<TilemapFile>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["itm"]
    }
}

impl TilemapFile {
    /// Loads a tilemap file from the filesystem.
    pub fn load(file_path: &OsStr) -> Result<Self> {
        let contents = fs::read_to_string(file_path)?;

        let file: TilemapFile = ron::from_str(&contents)?;

        Ok(file)
    }

    /// Saves the tilemap file to the filesystem.
    pub fn save(&self, file_path: &OsStr) -> Result<()> {
        let serialized = ron::to_string(&self)?;

        let mut file = File::create(file_path)?;
        file.write_all(serialized.as_bytes())?;

        Ok(())
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn source_image(&self) -> String {
        self.source_image.clone()
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn tile_height(&self) -> f32 {
        self.tile_height
    }

    pub fn tile_width(&self) -> f32 {
        self.tile_width
    }

    pub fn layers(&self) -> Vec<LayerDefinition> {
        self.layers.clone()
    }
}

impl LayerDefinition {
    pub fn order_id(&self) -> usize {
        self.order_id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn tiles(&self) -> Vec<Vec<i32>> {
        self.tiles.clone()
    }
}
