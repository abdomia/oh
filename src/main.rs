use csv;
use std::{ffi::OsString, process};
use clap::Parser;

#[derive(Debug, Parser)]
struct Oh {
    #[arg(value_name = "FILE")]
    file: OsString
}

fn read_csv(file: OsString) -> Result<(), csv::Error> {
    let mut reader = csv::Reader::from_path(file)?;
    for rec in reader.records() {
        match rec {
            Ok(v) => println!("{:?}", v),
            Err(_) => println!("error defined here man oooh")
        }
    }
    Ok(())
}

fn main() {
    let oh = Oh::parse();
    let file = oh.file;
    if let Err(_) = read_csv(file) {
        process::exit(1);
    }
}

