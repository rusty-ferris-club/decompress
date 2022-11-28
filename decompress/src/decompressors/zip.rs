use std::{
    fs::File,
    io::{self, BufReader},
    path::{Path, PathBuf},
};

use lazy_static::lazy_static;
use regex::Regex;
use zip::ZipArchive;

use crate::{DecompressError, Decompression, Decompressor, ExtractOpts};

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?i)\.zip$").unwrap();
}

#[derive(Default)]
pub struct Zip {
    re: Option<Regex>,
}
impl Zip {
    #[must_use]
    pub fn new(re: Option<Regex>) -> Self {
        Self { re }
    }
    #[must_use]
    pub fn build(re: Option<Regex>) -> Box<Self> {
        Box::new(Self::new(re))
    }
}

impl Decompressor for Zip {
    fn test(&self, archive: &Path) -> bool {
        archive
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .map_or(false, |f| self.re.as_ref().unwrap_or(&*RE).is_match(f))
    }

    fn decompress(
        &self,
        archive: &Path,
        to: &Path,
        opts: &ExtractOpts,
    ) -> Result<Decompression, DecompressError> {
        use std::fs;
        let mut rdr = ZipArchive::new(BufReader::new(File::open(archive)?))
            .map_err(|err| DecompressError::Error(err.to_string()))?;

        if !to.exists() {
            fs::create_dir_all(to)?;
        }
        for i in 0..rdr.len() {
            let mut file = rdr
                .by_index(i)
                .map_err(|err| DecompressError::Error(err.to_string()))?;
            let filepath = file
                .enclosed_name()
                .ok_or_else(|| DecompressError::Error("Invalid file path".to_string()))?;

            // strip prefixed components. this can be 0 parts, in which case strip does not happen.
            // it's done for when archives contain an enclosing folder
            let filepath = filepath.components().skip(opts.strip).collect::<PathBuf>();

            // because we potentially stripped a component, we may have an empty path, in which case
            // the joined target will be identical to the target folder
            // we take this approach to avoid hardcoding a check against empty ""
            let outpath = to.join(filepath);
            if outpath == to {
                continue;
            }

            if file.name().ends_with('/') {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = fs::File::create(&outpath)?;
                io::copy(&mut file, &mut outfile)?;
            }
            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&outpath, fs::Permissions::from_mode(mode))?;
                }
            }
        }
        Ok(Decompression { id: "zip" })
    }
}
