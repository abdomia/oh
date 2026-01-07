use std::process;

use clap::Parser;
use csv::StringRecord;
use crate::cli::cmds::handler::OhCommands;

pub type Header = StringRecord;
pub type Record = Vec<StringRecord>;

#[derive(Debug, Parser)]
pub struct Oh {
    #[command(subcommand)]
    cmd: Option<OhCommands>,
}

impl Oh {
    pub fn new() -> Self {
        Oh {
            cmd: None,
        }
    }
    // TODO make some kind of `error.rs` to handle errors globaly
    pub fn execute(self) -> OhCommands {
        let parsing = Oh::parse();
        match parsing.cmd {
            Some(cmd) => cmd,
            None => process::exit(1)
        }
    }
}

