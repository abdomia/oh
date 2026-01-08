use std::ffi::OsString;
use clap::Args;

use crate::{cli::cmds::{handler::{FileHandler, SelectBy}, output::OutputForm}, log::{data_handler::Data, logger::DataToLog}, reader::web_or_disk_reader};
use crate::log::data_handler::Record;


#[derive(Debug, Clone, Args)]
pub struct Selectors {
    #[arg(long = "index-col", short = 'i', required = false)]
    index: Option<usize>,

    #[arg(long = "range", short = 'r', required = false)]
    range: Option<OsString>,

    #[arg(long = "name", short = 'n', required = false)]
    name: Option<OsString>,
}

#[derive(Debug, Clone)]
pub struct OhRange {
    pub data: Data,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone)]
pub struct RangeInput {
    pub range: OsString,
    pub holder: Vec<String>
}

pub enum SelectState {
    Index,
    Range,
    Name,
    NoState
}

impl Selectors {
    pub fn selector_state(&self) -> SelectState {
        let indx = self.index;
        let rng = &self.range;
        let name = &self.name;

        match (indx, rng, name) {
            (Some(_), None, None) => SelectState::Index,
            (None, Some(_), None) => SelectState::Range,
            (None, None, Some(_)) => SelectState::Name,
            _ => SelectState::NoState
        }
    }

    pub fn selector_handler(self, state: SelectState, file: FileHandler) -> Result<DataToLog, String> {
        let data = web_or_disk_reader((file.web_file, file.disk_file))
            .expect("failed to read from web or disk be sure that you give the correct file path or url path");
        match state {
            SelectState::Index => {
                Err("".to_string())
            },
            SelectState::Range => {
                let check_range = RangeInput::new(self.range.unwrap())
                    .parse_range();
                if let Some(range) = check_range {
                    // for now implement range at the basic and simplest method
                    return OhRange::new(
                        data,
                        range.0,
                        range.1
                    ).prepare_for_log_with_range();
                } else {
                    Err("".to_string())
                }
            },
            SelectState::Name => {
                Err("".to_string())
            },
            SelectState::NoState => {
                Err("you must specifiy something!".to_string())
            },
        }
    }
}

impl OhRange {
    pub fn new(
        data: Data,
        s: usize,
        e: usize
    ) -> Self {
        OhRange {
            data: data,
            start: s,
            end: e
        }
    }

    pub fn prepare_for_log_with_range(
        &mut self,
    ) -> Result<DataToLog , String> {
        if self.start > self.data.rows.len()
        || self.start > self.end {
            Err(
                "either the `starting Range` is > rows length, `ending Range` is < rows length or the starting Range > ending Range."
                    .to_string()
            )
        } else {
            let desired: Record = self
                .data
                .rows
                .get(self.start..self.end)
                .unwrap()
                .to_vec();

            let data_to_log = DataToLog {
                // TODO make Data::new() function to handle this situation.
                data: Data {
                    header: self.data.header.clone(),
                    rows: desired,
                    limit_to_show: 10
                }
            };
            return Ok(data_to_log);
        }
    }
}

impl RangeInput {
    pub fn new(selection_str: OsString) -> Self {
        let holder = selection_str
            .clone()
            .into_string()
            .unwrap()
            .split(':')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

            RangeInput {
                range: selection_str,
                holder: holder
        }
    }


    //TODO validate the ranges that cames from the user function
    //TODO make -rc `range` `range | name`
    pub fn parse_range(self) -> Option<(usize, usize)> {
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

        Some((begin, finish))
    }
}

fn run_selector(selector: Selectors, file: FileHandler) -> Result<DataToLog, String> {
    let selector = Selectors {
        index: selector.index,
        range: selector.range,
        name: selector.name
    };
    let state = selector.selector_state();
    let sel_data = selector.selector_handler(state, file)?;
    Ok(sel_data)
}

pub fn handle_select_cmd(
    file: FileHandler,
    out: OutputForm,
    select_data: SelectBy
) {
    match select_data {
        SelectBy::Row(selectors) => {
            if let Ok(ready_data) = run_selector(selectors, file) {
                out.deside_output_format_and_log(ready_data);
            }
        },
        SelectBy::Col(selectors) => {
            if let Ok(ready_data) = run_selector(selectors, file) {
                out.deside_output_format_and_log(ready_data);
            }
        }
    }
}

