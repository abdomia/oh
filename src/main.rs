mod logging;
mod reader;

use reader::read_csv;
use clap::Parser;
use std::{ffi::OsString, process};
use crate::logging::log_csv_table;

#[derive(Debug, Parser)]
struct Oh {
    // TODO make this takes any number of args(files)
    #[arg(
        value_name = "FILE",
        num_args = 1
    )]
    file: OsString
}

fn main() {
    let oh = Oh::parse();
    let file = oh.file;
    // TODO make some kind of `error.rs` to handle this situation
    if let Ok((h, r)) = read_csv(file) {
        log_csv_table((h, r));
    } else {
        process::exit(1)
    }
}

