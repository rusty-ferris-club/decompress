//! Decompress an archive, supporting multiple format and stripping path
//! prefixes
//!
//! ## Usage
//! You can use the default `decompress` stack:
//! ```ignore
#![doc = include_str!("../examples/unpack.rs")]
//!
//! ```
//! Or build your own stack:
//! ```ignore
#![doc = include_str!("../examples/unzip.rs")]
//!
//! ```
//! __NOTE__: to include or exclude [decompressors] types, use `--features` and disable default.
//! This in turn removes or includes the (costly) dependencies these features need.
//!
//!
//!
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::module_name_repetitions)]
pub mod decompressors;

use derive_builder::Builder;
use std::{convert::Infallible, io, path::Path};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecompressError {
    #[error("could not decompress: `{0}`")]
    IO(#[from] io::Error),

    #[error("could not decompress: `{0}`")]
    Error(String),

    #[error("could not decompress: `{0}`")]
    Infallible(#[from] Infallible),

    #[error("no compressor found")]
    MissingCompressor,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct ExtractOpts {
    pub strip: usize,

    #[builder(setter(custom), default = "Box::new(|_| true)")]
    pub filter: Box<dyn Fn(&Path) -> bool>,
}

impl ExtractOptsBuilder {
    pub fn filter(mut self, value: impl Fn(&Path) -> bool + 'static) -> ExtractOptsBuilder {
        self.filter = Some(Box::new(value));
        self
    }
}

#[derive(Debug)]
pub struct Decompression {
    pub id: &'static str,
}

pub trait Decompressor {
    fn test(&self, archive: &Path) -> bool;
    ///
    /// Decompress an archive
    /// # Errors
    ///
    /// This function will return an error
    fn decompress(
        &self,
        archive: &Path,
        to: &Path,
        opts: &ExtractOpts,
    ) -> Result<Decompression, DecompressError>;
}

pub struct Decompress {
    decompressors: Vec<Box<dyn Decompressor>>,
}

impl Default for Decompress {
    fn default() -> Self {
        Self {
            decompressors: vec![
                #[cfg(feature = "zip")]
                Box::<decompressors::zip::Zip>::default(),
                #[cfg(feature = "targz")]
                Box::<decompressors::targz::Targz>::default(),
                #[cfg(feature = "tarball")]
                Box::<decompressors::tarball::Tarball>::default(),
                #[cfg(feature = "tarxz")]
                Box::<decompressors::tarxz::Tarxz>::default(),
                #[cfg(feature = "tarbz")]
                Box::<decompressors::tarbz::Tarbz>::default(),
                #[cfg(feature = "tarzst")]
                Box::<decompressors::tarzst::Tarzst>::default(),
                // order is important, `gz` is placed only after the targz variant did not match
                // if it's placed above targz, it will unpack and leave a tar archive.
                #[cfg(feature = "gz")]
                Box::<decompressors::gz::Gz>::default(),
                #[cfg(feature = "ar")]
                Box::<decompressors::ar::Ar>::default(),
                #[cfg(feature = "bz2")]
                Box::<decompressors::bz2::Bz2>::default(),
                #[cfg(feature = "xz")]
                Box::<decompressors::xz::Xz>::default(),
                #[cfg(feature = "zstd")]
                Box::<decompressors::zstd::Zstd>::default(),
            ],
        }
    }
}

impl Decompress {
    #[must_use]
    pub fn build(decompressors: Vec<Box<dyn Decompressor>>) -> Self {
        Self { decompressors }
    }

    /// Decompress
    ///
    /// # Errors
    ///
    /// This function will return an error if an IO or parsing error happened
    pub fn decompress<P: AsRef<Path>>(
        &self,
        archive: P,
        to: P,
        opts: &ExtractOpts,
    ) -> Result<Decompression, DecompressError> {
        if let Some(dec) = self
            .decompressors
            .iter()
            .find(|dec| dec.test(archive.as_ref()))
        {
            return dec.decompress(archive.as_ref(), to.as_ref(), opts);
        }
        Err(DecompressError::MissingCompressor)
    }
}

/// Decompress an archive with default decompressor set up
///
/// # Errors
///
/// This function will return an error if IO or parsing failed
pub fn decompress<P: AsRef<Path>>(
    archive: P,
    to: P,
    opts: &ExtractOpts,
) -> Result<Decompression, DecompressError> {
    Decompress::default().decompress(archive, to, opts)
}
