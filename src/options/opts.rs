use clap::{Subcommand, ValueEnum};
use std::ffi::OsString;
use crate::parser::parse_range;

// TODO implement all of these + custom error messages
#[derive(Debug, Subcommand)]
pub enum OhOptions {
    Select {
        // 1
        #[arg(long = "indx", short = 'i')]
        index: Option<usize>,

        // 2
        #[arg(long = "col-name", short = 'c')]
        col_name: Option<OsString>,

        // 3
        #[arg(long = "range", short = 'r', value_parser = parse_range)]
        range: (Option<String>, Option<String>),

        // 4
        #[arg(long, value_enum)]
        show_format: Option<FormatType>
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum FormatType {
    Csv, // implement this for now.
    // all the below will be implemented later on.
    Json,
    Yaml,
}

