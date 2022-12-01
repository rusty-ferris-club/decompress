use std::{fs, path::Path};

use decompress::{decompressors, Decompress, DecompressError, Decompression, ExtractOpts};
use dircmp::Comparison;
use regex::Regex;
use rstest::rstest;

#[rstest]
#[case("inner.tar", "inner_0", 0)]
#[case("bare.zip", "bare_zip_0", 0)]
#[case("bare.zip", "bare_zip_1", 1)]
#[case("bare.tar.gz", "bare_tgz_0", 0)]
#[case("bare.tar.gz", "bare_tgz_1", 1)]
#[case("bare.tar.xz", "bare_txz_0", 0)]
#[case("bare.tar.xz", "bare_txz_1", 1)]
#[case("folders.zip", "folders_zip_0", 0)]
#[case("folders.zip", "folders_zip_1", 1)]
#[case("folders.tar.gz", "folders_tgz_0", 0)]
#[case("folders.tar.gz", "folders_tgz_1", 1)]
#[case("folders.tar.xz", "folders_txz_0", 0)]
#[case("folders.tar.xz", "folders_txz_1", 1)]
#[case("inner.zip", "inner_zip_0", 0)]
#[case("inner.zip", "inner_zip_1", 1)]
#[case("inner.tar.gz", "inner_tgz_0", 0)]
#[case("inner.tar.gz", "inner_tgz_1", 1)]
#[case("inner.tar.xz", "inner_txz_0", 0)]
#[case("inner.tar.xz", "inner_txz_1", 1)]
#[case("inner.tar.zst", "inner_zst_1", 1)]
#[case("inner.tar.bz2", "inner_bz2_1", 1)]
#[case("bare.ar", "bare_ar", 0)]
#[case("sub.txt.gz", "gz_1", 0)]
#[trace]
fn test_archives(#[case] archive: &str, #[case] outdir: &str, #[case] strip: usize) {
    dec_test(&Decompress::default(), archive, outdir, strip).unwrap();
}

#[test]
fn test_custom() {
    let dec = Decompress::build(vec![decompressors::targz::Targz::build(Some(
        Regex::new(r"(?i)\.tzz$").unwrap(),
    ))]);
    let res = dec_test(&dec, "tar-gz.tzz", "custom_tar_gz_tzz", 0).unwrap();
    assert_eq!(res.id, "targz");

    // we swapped our decompressor stack, so now tar.gz should not work
    let res = dec_test(&dec, "bare.tar.gz", "bar_no_go", 0);
    match res {
        Err(DecompressError::MissingCompressor) => {}
        _ => panic!("should have not decompressed"),
    }
}

fn dec_test(
    decompressor: &Decompress,
    archive: &str,
    outdir: &str,
    strip: usize,
) -> Result<Decompression, DecompressError> {
    let from = format!("tests/fixtures/{}", archive);

    // poor man's setup: empty folders can't appear in github
    vec!["bare_zip_1", "bare_tgz_1", "bare_txz_1"]
        .iter()
        .map(|p| format!("tests/expected/{}", p))
        .for_each(|p| {
            if !Path::new(&p).exists() {
                let _res = fs::create_dir_all(&p);
            }
        });

    let to = format!("tests/out/{}", outdir);
    if Path::new(&to).exists() {
        let _res = fs::remove_dir_all(&to);
    }

    let res = decompressor.decompress(&from, &to, &ExtractOpts { strip });
    let res = res?;

    // need to do a 2way for full comparison.
    let diff = Comparison::default();

    let result = diff
        .compare(
            Path::new(&to),
            Path::new(&format!("tests/expected/{}", outdir)),
        )
        .unwrap();
    println!("{:?}", result);
    assert!(result.is_empty());
    Ok(res)
}
