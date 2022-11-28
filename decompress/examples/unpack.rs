#![allow(clippy::cognitive_complexity)]
use clap::{arg, command};
use decompress::ExtractOpts;

fn main() {
    let matches = command!()
        .arg(arg!(<archive> "Archive to extract"))
        .arg(arg!(<out> "Output folder"))
        .arg(arg!(
            -s --strip "Strip the first component of the archive"
        ))
        .get_matches();

    let archive = matches.get_one::<String>("archive").expect("required");
    let to = matches.get_one::<String>("out").expect("required");
    let strip = usize::from(matches.get_flag("strip"));
    let res = decompress::decompress(archive, to, &ExtractOpts { strip });
    println!("{:?}", res);
}
