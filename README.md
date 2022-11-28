
# Decompress

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


# Copyright

Copyright (c) 2022 [@jondot](http://twitter.com/jondot). See [LICENSE](LICENSE.txt) for further details.
