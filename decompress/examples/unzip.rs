#![allow(clippy::cognitive_complexity)]
use clap::{arg, command};
use decompress::{decompressors, ExtractOpts};
use regex::Regex;

fn main() {
    let matches = command!()
        .arg(arg!(<archive> "Archive to Unzip (attempt any file)"))
        .arg(arg!(<out> "Output folder"))
        .arg(arg!(
            -s --strip "Strip the first component of the archive"
        ))
        .get_matches();

    let archive = matches.get_one::<String>("archive").expect("required");
    let to = matches.get_one::<String>("out").expect("required");
    let strip = usize::from(matches.get_flag("strip"));
    let decompressor = decompress::Decompress::build(vec![decompressors::zip::Zip::build(Some(
        Regex::new(r".*").unwrap(),
    ))]);
    let res = decompressor.decompress(archive, to, &ExtractOpts { strip });
    println!("{:?}", res);
}
