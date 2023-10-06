use std::path::Path;

use bevy::asset::{AssetLoader, LoadedAsset};

use super::{Error, tilemap::TilemapDefinition};

pub trait Loader {
    fn load(&mut self, path: &Path) -> Result<(), Error>;
    fn unload(&mut self, name: &str);
}


pub struct TilemapDefinitionCollection();

#[derive(Default)]
pub struct TilemapAssetLoader;

impl AssetLoader for TilemapAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<TilemapDefinition>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["itm"]
    }
}