use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use thiserror::Error;

pub trait AssetLoader {
    type AssetError: std::error::Error;

    /// Fetch the named texture, if available.
    fn get_texture(&self, name: &str) -> Result<&image::RgbImage, Self::AssetError>;
}

#[derive(Debug, Error)]
pub enum PreloadError {
    #[error("Cannot load texture {0}: {1}")]
    Load(PathBuf, image::ImageError),
    #[error("Cannot found texture with name: {0}")]
    NotFound(String),
}

pub struct PreloadedImages {
    images: HashMap<String, image::RgbImage>,
}

impl PreloadedImages {
    pub fn new() -> Self {
        PreloadedImages {
            images: HashMap::new(),
        }
    }

    pub fn load_file<P: AsRef<Path>>(&mut self, name: &str, path: P) -> Result<(), PreloadError> {
        let image = image::open(&path)
            .map(|img| img.flipv().to_rgb8())
            .map_err(|e| PreloadError::Load(path.as_ref().to_owned(), e))?;

        self.images.insert(name.to_owned(), image);
        Ok(())
    }

    pub fn load_files<'a, P: AsRef<Path>, I: IntoIterator<Item = (&'a str, P)>>(
        &mut self,
        files: I,
    ) -> Result<(), PreloadError> {
        let _res = files
            .into_iter()
            .map(|(name, path)| self.load_file(name, path))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }
}

impl AssetLoader for PreloadedImages {
  type AssetError = PreloadError;

  fn get_texture(&self, name: &str) -> Result<&image::RgbImage, Self::AssetError> {
    self.images.get(name).ok_or(PreloadError::NotFound(name.to_owned()))
  }
}