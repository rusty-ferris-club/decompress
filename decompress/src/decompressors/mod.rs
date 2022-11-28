#[cfg(feature = "tarball")]
mod tar_common;

#[cfg(feature = "tarball")]
pub mod tarball;

#[cfg(feature = "tarzst")]
pub mod tarzst;

#[cfg(feature = "tarxz")]
pub mod tarxz;

#[cfg(feature = "tarbz")]
pub mod tarbz;

#[cfg(feature = "targz")]
pub mod targz;

#[cfg(feature = "zip")]
pub mod zip;

#[cfg(feature = "ar")]
pub mod ar;
