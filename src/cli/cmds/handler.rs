use clap::{Subcommand, Args};
use std::{ffi::OsString, path::PathBuf};

use crate::cli::cmds::select::{handle_select_cmd, SelectBy};
use crate::cli::cmds::get::handle_get_cmd;
use crate::cli::cmds::output::OutputForm;

#[derive(Debug, Clone, Args)]
pub struct FileHandler {
    // make this takes any number of args(files)...
    pub disk_file: Option<PathBuf>,
    // test for edge cases like swap the `required_unless_present` if not working
    #[arg(
        value_name = "SOURCE",
        required_unless_present = "disk_file",
        conflicts_with = "disk_file",
        long = "external-data",
        short = 'w'
    )]
    pub web_file: Option<OsString>,
}

// TODO implement all of these + custom error messages
#[derive(Debug, Clone, Subcommand)]
pub enum OhCommands {
    Select {
        #[command(flatten)]
        file: FileHandler,
        #[command(flatten)]
        selection_data: SelectBy,
        // TODO move the argument output_shape to Oh struct.
        #[arg(long = "output", short = 'o', value_enum, default_value="table")]
        output_shape: OutputForm
    },
    Get {
        #[command(flatten)]
        file: FileHandler,
        #[arg(long = "output", short = 'o', value_enum, default_value="table")]
        output_shape: OutputForm
    }
}

impl OhCommands {
    pub fn handle_cmds(self) {
        match self {
            OhCommands::Select {
                file: f,
                selection_data: selection,
                output_shape: out
            } => {
                handle_select_cmd(f, out, selection)
            },
            OhCommands::Get {
                file: f,
                output_shape: out
            } => {
                handle_get_cmd(f, out);
            },
        }
    }
}

