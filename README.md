
# Decompress

[<img alt="github" src="https://img.shields.io/badge/github-rusty_ferris_club/decompress-8dagcb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/rusty-ferris-club/decompress)
[<img alt="crates.io" src="https://img.shields.io/crates/v/decompress.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/decompress)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-decompress-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/decompress)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/rusty-ferris-club/decompress/Build/master?style=for-the-badge" height="20">](https://github.com/rusty-ferris-club/decompress/actions?query=branch%3Amaster)

A library that supports decompression of archives in multiple formats, inspired by ergonomics from Node's [decompress](https://github.com/kevva/decompress).

* Includes a default stack of decompressors supporting: `zip`, `tar`, `tar.gz`, `tar.bz2`, `tar.xz`, `tar.zst` (zstd compression), `ar` (Unix Archive)
* Build your own decompressors and add them
* Compose a custom stack (exclude compressors, respond to different file extensions)
* Use `cargo` features to avoid compiling formats you don't need

# Dependency

```toml
[dependencies]
decompress = "0.1.0"
```


# Usage

Default use:

```rust
decompress::decompress(archive, to, &ExtractOpts::default());
```

Strip the first component of all paths in the archive (for when you have a wrapper folder you don't need):

```rust
decompress::decompress(archive, to, &ExtractOpts{ strip: 1 });
```

A micro optimization:

```rust
let decompressor = decompress::Decompress::default()
// use decompressor
// decompressor.decompress(...)
```

Build your own stack:

```rust
use regex::Regex;
let decompressor = decompress::Decompress::build(vec![decompressors::zip::Zip::build(Some(
    Regex::new(r".*").unwrap(),
))]);
// use decompressor
// decompressor.decompress(...)
```

It's also possible to filter unwanted files, similar to [nodejs decompress](https://github.com/kevva/decompress)
```rust
let decompressor = decompress::Decompress::default();
let res = decompressor.decompress(
    archive,
    to,
    &ExtractOptsBuilder::default()
        .strip(strip)
        .filter(|path| {
            if let Some(path) = path.to_str() {
            return path.ends_with("abc.sh");
            }
            false
        })
        .build()
        .unwrap(),
);
```


# Copyright

Copyright (c) 2022 [@jondot](http://twitter.com/jondot). See [LICENSE](LICENSE.txt) for further details.
