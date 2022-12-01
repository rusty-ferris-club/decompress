use crate::{DecompressError, Decompression, Decompressor, ExtractOpts};
use lazy_static::lazy_static;
use regex::Regex;
use std::{fs, io};
use std::{fs::File, io::BufReader, path::Path};

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?i)\.gz$").unwrap();
}

#[derive(Default)]
pub struct Gz {
    re: Option<Regex>,
}

impl Gz {
    #[must_use]
    pub fn new(re: Option<Regex>) -> Self {
        Self { re }
    }
    #[must_use]
    pub fn build(re: Option<Regex>) -> Box<Self> {
        Box::new(Self::new(re))
    }
}

impl Decompressor for Gz {
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
        _opts: &ExtractOpts,
    ) -> Result<Decompression, DecompressError> {
        let fd = BufReader::new(File::open(archive)?);
        let dec = flate2::bufread::GzDecoder::new(fd);
        if !Path::new(to).exists() {
            let _res = fs::create_dir_all(to);
        }
        let mut outfile =
            fs::File::create(&to.join(archive.file_stem().ok_or_else(|| {
                DecompressError::Error("cannot compose a file name".to_string())
            })?))?;

        io::copy(&mut BufReader::new(dec), &mut outfile)?;
        Ok(Decompression { id: "gz" })
    }
}
