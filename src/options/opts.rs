// TODO make a flags and parses them to do the follwoing:
// 1. make start_rows, end_rows variables debending on the user input to handle filtering options
use clap::{Subcommand, ValueEnum};
use std::{ffi::OsString};

use crate::{data::data::Data, log::OutputForm};

// TODO implement all of these + custom error messages
#[derive(Debug, Subcommand, Clone)]
pub enum OhOptions {
    Select {
        // 1
        #[arg(long = "indx", short = 'i')]
        index: Option<usize>,
        // 2
        #[arg(long = "col-name", short = 'c')]
        col_name: Option<OsString>,
        // 3
        #[arg(long = "range", short = 'r', value_parser = parse_range, num_args = 1..=2)]
        range: Data,
        // 4
        #[arg(long = "output", short = 'o', value_enum)]
        output_form: OutputForm
    },
}

#[derive(Debug, Clone)]
pub struct RangeInput {
    pub range: String
}

#[derive(Debug, Clone)]
pub struct Selection {
    pub holder: Vec<String>
}

#[derive(Debug, Clone)]
pub enum Points {
    Row((usize, usize)),
    Col((usize, usize))
}

impl RangeInput {
    pub fn new(selection_str: &str) -> Self {
        RangeInput {
            range: selection_str.to_string()
        }
    }

    pub fn split_selection(self) -> Selection {
        Selection {
            holder:
                self
                    .range
                    .split(|p| p == ' ' || p == ':')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
        }
    }

}

impl Selection {
    // TODO validate the ranges that cames from the user
    pub fn calculate_points(self) -> (Option<Points>, Option<Points>) {
        return match self.holder.len() {
            2 => {
                let start_val_row = self.holder.iter().nth(0).expect("coudn't find start_val");
                let end_val_row = self.holder.iter().nth(1).expect("coudn't find start_val");

                (
                    Some(
                        Points::Row((
                            start_val_row.parse::<usize>().expect("couldn't parse the `start_val_row` range"),
                            end_val_row.parse::<usize>().expect("couldn't parse the `end_val_row` range")
                        )
                )),
                    None
                )
            },
            4 => {
                let start_val_row = self.holder.iter().nth(0).expect("coudn't find start_val");
                let end_val_row = self.holder.iter().nth(1).expect("coudn't find start_val");

                let start_val_col = self.holder.iter().nth(2).expect("coudn't find start_val");
                let end_val_col = self.holder.iter().nth(3).expect("coudn't find start_val");
                (
                    Some(
                        Points::Row((
                            start_val_row.parse::<usize>().expect("couldn't parse the `start_val_row` range"),
                            end_val_row.parse::<usize>().expect("couldn't parse the `end_val_row` range"))
                        )),
                    Some(
                        Points::Col((
                            start_val_col.parse::<usize>().expect("couldn't parse the `start_val_row` range"),
                            end_val_col.parse::<usize>().expect("couldn't parse the `end_val_row` range"))
                        ))
                )
            },
            _ => (None, None)
        };
    }
}

fn parse_range(input: &str) -> Result<Data, String> {
    // FIX this okay..
    let v = RangeInput::new(input)
        .split_selection()
        .calculate_points();
}
