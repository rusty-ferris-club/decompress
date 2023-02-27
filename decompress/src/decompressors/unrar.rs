use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;

use crate::{DecompressError, Decompression, Decompressor, ExtractOpts, Listing};

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?i)\.rar$").unwrap();
}

#[derive(Default)]
pub struct Unrar {
    re: Option<Regex>,
}
impl Unrar {
    #[must_use]
    pub fn new(re: Option<Regex>) -> Self {
        Self { re }
    }
    #[must_use]
    pub fn build(re: Option<Regex>) -> Box<Self> {
        Box::new(Self::new(re))
    }
}

impl Decompressor for Unrar {
    fn test_mimetype(&self, archive: &str) -> bool {
        archive == "application/vnd.rar"
    }

    fn test(&self, archive: &Path) -> bool {
        archive
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .map_or(false, |f| self.re.as_ref().unwrap_or(&*RE).is_match(f))
    }

    fn list(&self, archive: &Path) -> Result<Listing, DecompressError> {
        let res = unrar::Archive::new(archive.to_string_lossy().to_string())
            .list()
            .map_err(|e| DecompressError::Error(e.to_string()))?
            .process()
            .map_err(|e| DecompressError::Error(e.to_string()))?;

        Ok(Listing {
            id: "rar",
            entries: res
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>(),
        })
    }

    fn decompress(
        &self,
        archive: &Path,
        to: &Path,
        _opts: &ExtractOpts,
    ) -> Result<Decompression, DecompressError> {
        use std::fs;
        if !to.exists() {
            fs::create_dir_all(to)?;
        }

        let res = unrar::Archive::new(archive.to_string_lossy().to_string())
            .extract_to(to.to_string_lossy().to_string())
            .map_err(|e| DecompressError::Error(e.to_string()))?
            .process()
            .map_err(|e| DecompressError::Error(e.to_string()))?;

        Ok(Decompression {
            id: "rar",
            files: res
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>(),
        })
    }
}
