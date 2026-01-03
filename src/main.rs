mod logging;

use csv::{self, StringRecord};
use std::{ffi::OsString, process};
use clap::Parser;

use crate::logging::logging_csv;

#[derive(Debug, Parser)]
struct Oh {
    // TODO make this takes any number of args(files)
    #[arg(
        value_name = "FILE",
        num_args = 0..=1
    )]
    file: OsString
}

fn read_csv(file: OsString) -> Result<(StringRecord, Vec<StringRecord>), csv::Error> {
    let r = csv::Reader::from_path(file);
    let mut reader = match r {
        Ok(v) => v,
        Err(e) => return Err(e)
    };

    let mut rows: Vec<StringRecord> = vec![];
    for vs in reader.records() {
        if let Ok(v) = vs {
            rows.push(v);
        }
    };
    let headers = reader.headers();
    return match headers {
        Ok(h) => {
            Ok((h.clone(), rows))
        },
        Err(e) => Err(e)
    };
}

fn main() {
    let oh = Oh::parse();
    let file = oh.file;
    if let Ok((h, r)) = read_csv(file) {
        logging_csv((h, r));
    } else {
        process::exit(1)
    }
}

