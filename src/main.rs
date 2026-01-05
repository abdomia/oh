mod log;
mod reader;
mod options;
mod parser;

use reader::read_csv;
use clap::{Parser};
use std::{path::PathBuf, process};
use crate::log::{log_csv_table, prepare_for_log};
use crate::options::opts::OhOptions;

pub struct RangeSelection {
    pub range_selection: (Option<String>, Option<String>)
}

#[derive(Debug, Parser)]
struct Oh {
    // TODO make this takes any number of args(files)...
    #[arg(value_name = "FILE", num_args = 1)]
    file: PathBuf,
    #[command(subcommand)]
    filter_opts: Option<OhOptions>
}

impl Oh {
    fn new() -> Self {
        Oh {
            file: "".into(),
            filter_opts: None
        }
    }

    // TODO make some kind of `error.rs` to handle this situation
    fn cli(mut self) {
        let fields = Oh::parse();
        self.file = fields.file;
        if let Ok((h, r)) = read_csv(self.file) {
            // TODO handle such case that if the user want to print it to diffrent formats.
            match fields.filter_opts {
                Some(opts) => {
                    match opts {
                        OhOptions::Select { index: indx, .. } => {
                            if let Some(v) = indx {
                                let desired_row = r.iter().nth(v).expect("there is no row with that index!");
                            } else {
                                // simple log here cause the index is not specified
                                println!("
                            there is no `index` specified. please specifiy what index you want to select
                                ");
                            }
                        },
                        OhOptions::Select { col_name: cn, .. } => {
                            if let Some(v) = cn {
                                let name = h;
                            } else {
                            }
                        },
                        OhOptions::Select { range: rng, ..} => {
                        },
                    }
                },
                None => ()
            }
        } else {
            process::exit(1);
        }
    }

}

fn main() {
    Oh::new()
        .cli();
}

