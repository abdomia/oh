use clap::Subcommand;
use std::{ffi::OsString, path::PathBuf};

// TODO implement all of these + custom error messages
#[derive(Debug, Subcommand, Clone)]
pub enum OhCommands {
    Select {
        // TODO make this file field takes any number of args(files)...
        // 1
        #[arg(long = "row", short = 'i')]
        row_index: Option<usize>,
        // 2
        #[arg(long = "header", short = 'n')]
        header_name_or_index: Option<OsString>,
        // 3
        #[arg(long = "range", short = 'r', num_args = 1..=2)]
        range: Option<OsString>,
    },
    Show {
        #[arg(value_name = "FILE", required = true)]
        file: PathBuf,
    },
    Temp
}

impl OhCommands {
    pub fn handle_cmds(&self) {
        match self {
            OhCommands::Select {
                header_name_or_index: name_or_indx,
                row_index: indx,
                range: rng
            } => {
            },
            OhCommands::Show {
                file: file
            } => {
            },
            OhCommands::Temp => {}
        }
    }
}


