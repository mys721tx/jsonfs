use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("INPUT")
                .help("path to the JSON file")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("MOUNTPOINT")
                .help("path to the mounting point")
                .index(2)
                .required(true),
        )
        .get_matches();

    let path = Path::new(matches.value_of("INPUT").unwrap());
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let u: serde_json::Value = serde_json::from_reader(reader).unwrap();

    dbg!(u);
}
