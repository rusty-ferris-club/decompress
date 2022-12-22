use std::{fs, path::Path};

use decompress::{decompressors, Decompress, DecompressError, Decompression, ExtractOptsBuilder};
use dircmp::Comparison;
use regex::Regex;
use rstest::rstest;

#[rstest]
#[case("inner.tar", "inner_0", 0, "tarball")]
#[case("bare.zip", "bare_zip_0", 0, "zip")]
#[case("bare.zip", "bare_zip_1", 1, "zip")]
#[case("bare.tar.gz", "bare_tgz_0", 0, "targz")]
#[case("bare.tar.gz", "bare_tgz_1", 1, "targz")]
#[case("bare.tar.xz", "bare_txz_0", 0, "tarxz")]
#[case("bare.tar.xz", "bare_txz_1", 1, "tarxz")]
#[case("folders.zip", "folders_zip_0", 0, "zip")]
#[case("folders.zip", "folders_zip_1", 1, "zip")]
#[case("folders.tar.gz", "folders_tgz_0", 0, "targz")]
#[case("folders.tar.gz", "folders_tgz_1", 1, "targz")]
#[case("folders.tar.xz", "folders_txz_0", 0, "tarxz")]
#[case("folders.tar.xz", "folders_txz_1", 1, "tarxz")]
#[case("inner.zip", "inner_zip_0", 0, "zip")]
#[case("inner.zip", "inner_zip_1", 1, "zip")]
#[case("inner.tar.gz", "inner_tgz_0", 0, "targz")]
#[case("inner.tar.gz", "inner_tgz_1", 1, "targz")]
#[case("inner.tar.xz", "inner_txz_0", 0, "tarxz")]
#[case("inner.tar.xz", "inner_txz_1", 1, "tarxz")]
#[case("inner.tar.zst", "inner_zst_1", 1, "tarzst")]
#[case("inner.tar.bz2", "inner_bz2_1", 1, "tarbz")]
#[case("bare.ar", "bare_ar", 0, "ar")]
#[case("sub.txt.gz", "gz_1", 0, "gz")]
#[case("sub.txt.bz2", "bz_2", 0, "bz2")]
#[case("sub.txt.xz", "xz_1", 0, "xz")]
#[case("sub.txt.zst", "zstd_1", 0, "zst")]
#[trace]
fn test_archives(
    #[case] archive: &str,
    #[case] outdir: &str,
    #[case] strip: usize,
    #[case] id: &str,
) {
    vec!["bare_zip_1", "bare_tgz_1", "bare_txz_1"]
        .iter()
        .map(|p| format!("tests/expected/{p}"))
        .for_each(|p| {
            if !Path::new(&p).exists() {
                let _res = fs::create_dir_all(&p);
            }
        });

    let extract_opts = ExtractOptsBuilder::default().strip(strip).build().unwrap();

    let res = assertion(archive, outdir, |from, to| {
        Decompress::default().decompress(from, to, &extract_opts)
    })
    .unwrap();

    assert_eq!(res.id, id);
}

#[test]
fn test_custom() {
    let extract_opts = ExtractOptsBuilder::default().build().unwrap();
    let dec = Decompress::build(vec![decompressors::targz::Targz::build(Some(
        Regex::new(r"(?i)\.tzz$").unwrap(),
    ))]);

    let res = assertion("tar-gz.tzz", "custom_tar_gz_tzz", |from, to| {
        dec.decompress(from, to, &extract_opts)
    })
    .unwrap();
    assert_eq!(res.id, "targz");

    // we swapped our decompressor stack, so now tar.gz should not work
    let res = assertion("bare.tar.gz", "bar_no_go", |from, to| {
        dec.decompress(from, to, &extract_opts)
    });

    match res {
        Err(DecompressError::MissingCompressor) => {}
        _ => panic!("should have not decompressed"),
    }
}

#[rstest]
#[case("bare.tar.gz", "bare_filter_tgz_0", "targz")]
#[case("bare.zip", "bare_filter_zip_0", "zip")]
#[trace]
fn test_filter(#[case] archive: &str, #[case] outdir: &str, #[case] id: &str) {
    let extract_opts = ExtractOptsBuilder::default()
        .strip(0)
        .filter(|path| {
            if let Some(path) = path.to_str() {
                return path.ends_with("ex.sh");
            }
            false
        })
        .build()
        .unwrap();

    let res = assertion(archive, outdir, |from, to| {
        Decompress::default().decompress(from, to, &extract_opts)
    })
    .unwrap();

    assert_eq!(res.id, id);
}

#[rstest]
#[case("bare.tar.gz", "bare_map_tgz_0", "targz")]
#[case("bare.zip", "bare_map_zip_0", "zip")]
#[trace]
fn test_map(#[case] archive: &str, #[case] outdir: &str, #[case] id: &str) {
    let extract_opts = ExtractOptsBuilder::default()
        .strip(0)
        .map(|path| {
            let mut path = path.to_path_buf();
            path.set_file_name(format!(
                "abc-{}",
                path.file_name().unwrap().to_str().unwrap()
            ));
            path.into()
        })
        .build()
        .unwrap();

    let res = assertion(archive, outdir, |from, to| {
        Decompress::default().decompress(from, to, &extract_opts)
    })
    .unwrap();

    assert_eq!(res.id, id);
}

#[test]
fn test_can_decompress() {
    assert!(Decompress::default().can_decompress("foo/bar/baz.tar.gz"));
    assert!(!Decompress::default().can_decompress("foo/bar/baz.tar.foo"));
}

fn assertion(
    from: &str,
    to: &str,
    extract: impl Fn(&str, &str) -> Result<Decompression, DecompressError>,
) -> Result<Decompression, DecompressError> {
    let from = format!("tests/fixtures/{from}");
    let out = format!("tests/out/{to}");

    if Path::new(&out).exists() {
        fs::remove_dir_all(&out).unwrap();
    }

    let extraction = extract(from.as_str(), out.as_str())?;

    let result = Comparison::default()
        .compare(Path::new(&out), Path::new(&format!("tests/expected/{to}")))
        .unwrap();

    assert!(result.is_empty());

    Ok(extraction)
}
