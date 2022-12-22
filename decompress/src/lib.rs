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
use std::borrow::Cow;
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

type FilterFn = dyn Fn(&Path) -> bool;
type MapFn = dyn Fn(&Path) -> Cow<'_, Path>;

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct ExtractOpts {
    #[builder(default)]
    pub strip: usize,

    #[builder(setter(custom), default = "Box::new(|_| true)")]
    pub filter: Box<FilterFn>,

    #[builder(setter(custom), default = "Box::new(|path| Cow::from(path))")]
    pub map: Box<MapFn>,
}

impl ExtractOptsBuilder {
    /// Given a predicate, filter a path in.
    #[must_use]
    pub fn filter(mut self, value: impl Fn(&Path) -> bool + 'static) -> Self {
        self.filter = Some(Box::new(value));
        self
    }
    /// Given a mapping function, transform a path into a different or similar path
    #[must_use]
    pub fn map(mut self, value: impl Fn(&Path) -> Cow<'_, Path> + 'static) -> Self {
        self.map = Some(Box::new(value));
        self
    }
}

#[derive(Debug)]
pub struct Decompression {
    pub id: &'static str,
}

///
/// `Decompressor` is a trait that you can implement to add your own decompressor type.
/// A `Decompressor` is inserted into a stack, where given a potential archive file,
/// many decompressors may attempt to test if they're capable of unpacking it.
/// The first `Decompressor` which will test true will be the one selected to unpack.
///
/// It is _recommended_ to let a user pick a regex for testing against a `Path`, although
/// there is no limit to what you can do, as long as a user can override the Decompressor
/// decision when building a custom stack.
///
pub trait Decompressor {
    ///
    /// Test if this `Decompressor` can unpack an archive, given a path.
    /// The convention is to use `Regex` internally to test a path, because this is
    /// a convenient way for end users to override the behavior.
    /// You may choose to implement a different, but configurable, testing strategy.
    fn test(&self, archive: &Path) -> bool;
    ///
    /// Decompress an archive
    ///
    /// # Errors
    ///
    /// This function will return an error if unpacking fails.
    fn decompress(
        &self,
        archive: &Path,
        to: &Path,
        opts: &ExtractOpts,
    ) -> Result<Decompression, DecompressError>;
}

///
/// Represent a stack of decompressors with a default stack preconfigured when calling `new`
///
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
    /// Build given a custom stack of decompressors
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

    /// Returns `true` if any of the decompressors in the stack can decompress this
    /// specific archive
    pub fn can_decompress<P: AsRef<Path>>(&self, archive: P) -> bool {
        self.decompressors
            .iter()
            .any(|dec| dec.test(archive.as_ref()))
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
