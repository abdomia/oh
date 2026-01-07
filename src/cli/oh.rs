use std::process;
use std::ffi::OsString;

use clap::Parser;
use csv::StringRecord;
use crate::log::OutputForm;
use crate::cli::cmds::handler::OhCommands;

pub type Header = StringRecord;
pub type Record = Vec<StringRecord>;

#[derive(Debug, Parser)]
pub struct Oh {
    #[command(subcommand)]
    cmd: Option<OhCommands>,
    // 4
    #[arg(long = "output", short = 'o', value_enum, default_value="table")]
    output_form: OutputForm
}

#[derive(Debug, Clone)]
pub struct OhRange {
    pub start: usize,
    pub end: usize,
}


impl Oh {
    pub fn new() -> Self {
        Oh {
            cmd: None,
            output_form: OutputForm::Table
        }
    }
    // TODO make some kind of `error.rs` to handle errors globaly
    pub fn cli(self) -> OhCommands {
        let parsing = Oh::parse();
        match parsing.cmd {
            Some(cmd) => cmd,
            None => {
                process::exit(1)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RangeInput {
    pub range: OsString
}

#[derive(Debug, Clone)]
pub struct Selection {
    pub holder: Vec<String>
}

impl RangeInput {
    pub fn new(selection_str: Option<OsString>) -> Self {
        RangeInput {
            range: selection_str.expect("Please provide the selection value")
        }
    }

    pub fn split_selection(self) -> Selection {
        Selection {
            holder: self
                .range
                // TODO try split with .. range
                .into_string()
                .unwrap()
                .split(':')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        }
    }
}

impl Selection {
    //TODO validate the ranges that cames from the user function
    //TODO make -rc `range` `range | name`
    pub fn parse_range(self) -> Option<OhRange> {
        let [start, end] = self
            .holder
            .get(0..2)
            .expect("you have to specifiy the `start:end` range syntax") else {
            return None
        };
        // TODO consider usize::MAX and usize::MIN
        let begin: usize = start.parse()
            .expect("you should type a number between 0..any positve unsigned number");
        let finish: usize = end.parse()
            .expect("you should type a number between 0..any positve unsigned number");

        Some(OhRange {
            start: begin,
            end: finish
        })
    }
}

