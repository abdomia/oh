use clap::{Subcommand, Args, ValueEnum};
use std::{ffi::OsString, path::PathBuf};

use crate::{cli::cmds::get::handle_get_cmd, reader::{read_from_file_source}};

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputForm {
    Table, // this is the default
    Json,
    Yaml
}

pub enum State {
    Index,
    Range,
    Name
}

#[derive(Debug, Clone, Subcommand)]
pub enum SelectBy {
    Row {
        #[arg(long = "indx-row", short = 'i', required = false)]
        index: Option<usize>,

        #[arg(long = "range", short = 'r', required = false)]
        range: Option<OsString>,
    },
    #[command(subcommand_required = false)]
    Col {
        #[arg(long = "index-col", short = 'i', required = false)]
        index: Option<usize>,

        #[arg(long = "range", short = 'r', required = false)]
        range: Option<OsString>,

        #[arg(long = "name", short = 'n', required = false)]
        name: Option<OsString>,
    }
}

#[derive(Debug, Clone, Args)]
pub struct FileHandler {
    // make this takes any number of args(files)...
    disk_file: Option<PathBuf>,
    // test for edge cases like swap the `required_unless_present` if not working
    #[arg(
        value_name = "SOURCE",
        required_unless_present = "disk_file",
        conflicts_with = "disk_file",
        long = "external-data",
        short = 's'
    )]
    web_file: Option<OsString>,
}

#[derive(Debug, Clone, Args)]
pub struct FileOutput {
    #[arg(long = "output", short = 'o', value_enum, default_value="table")]
    file_format: OutputForm
}

// TODO implement all of these + custom error messages
#[derive(Debug, Subcommand, Clone)]
pub enum OhCommands {
    Select {
        #[command(flatten)]
        file: FileHandler,
        #[command(subcommand)]
        selection_data: SelectBy,
        #[command(flatten)]
        output_shape: FileOutput,
    },
    Get {
        #[command(flatten)]
        file: FileHandler,
        #[command(flatten)]
        output_shape: FileOutput,
    }
}

impl OhCommands {
    pub fn handle_cmds(self) {
        match self {
            OhCommands::Select {
                ..
            } => {
            },
            OhCommands::Get {
                file: f,
                output_shape: out
            } => {
                let reader = read_from_file_source((f.web_file, f.disk_file));
                if let Ok(r) = reader {
                    let handler = handle_get_cmd(r, out.file_format);
                    if let Ok(h) = handler {
                        println!("{}", h.1);
                    }
                }
            },
        }
    }
}

