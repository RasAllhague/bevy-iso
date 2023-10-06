use std::path::Path;

pub mod tileset;
pub mod tilemap;
pub mod loader;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Ron(ron::Error)
}

