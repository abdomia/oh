use std::ffi::OsString;
use clap::Args;
use csv::StringRecord;

use crate::{cli::cmds::{handler::{FileHandler}, output::OutputForm}, log::{data_handler::{Data, Header}, logger::DataToLog}, reader::web_or_disk_reader};
use crate::log::data_handler::Record;

#[derive(Debug, Clone, Args)]
pub struct SelectBy {
    #[arg(long, short = 'r')]
    row: Option<OsString>,
    #[arg(long, short = 'c')]
    col: Option<OsString>
}

impl SelectBy {
    // index_handler.
    // name_handler.
    fn compute_start_end(
        &self,
        rng: &OsString,
        data: &Data,
        by_row: bool,
        by_col: bool
    ) -> Option<(usize, usize)> {
        let mut holder = rng
            .to_str()
            .unwrap()
            .split(':')
            .map(|e| e.to_string());

        let start = holder.next().unwrap_or("0".to_string());
        if by_row {
            let end = holder.next().unwrap_or(format!("{}", data.rows.len()));
            let s = start.parse::<usize>().expect("failed to parse");
            let e = end.parse::<usize>().expect("failed to parse");
            let valid = self.validate(s, e, data).expect("error!");
            Some(valid)
        } else if by_col {
            let end = holder.next().unwrap_or(format!("{}", data.header.len()));
            let s = start.parse::<usize>().expect("failed to parse");
            let e = end.parse::<usize>().expect("failed to parse");
            let valid = self.validate(s, e, data).expect("error while something happend here!");
            Some(valid)
        } else {
            None
        }
    }

    fn entry_handler(&self, entry: &OsString, data: &Data, by_row: bool, by_col: bool) -> Result<DataToLog, String>  {
        let ent = entry.to_str();
        let index_choice = ent.unwrap().parse::<usize>();
        return match index_choice {
            Ok(i) => Err("".to_string()),
            Err(_) => {
                let range_choice = ent
                    .unwrap()
                    .contains(':');
                if range_choice {
                    let range = self.compute_start_end(entry, &data, by_row, by_col);
                    if let Some(r) = range {
                        self.handle_start_end_range(r.0, r.1, by_row, by_col, data)
                    } else {
                        Err("".to_string())
                    }
                } else {
                    Err("".to_string())
                }
            }
        };
    }

    pub fn handle_start_end_range(
        &self,
        start: usize,
        end: usize,
        by_row: bool,
        by_col: bool,
        data: &Data
    ) -> Result<DataToLog, String> {
        if by_row {
            let desired_row: Record = data
                .rows
                .get(start..end)
                .unwrap()
                .to_vec();

            let data_to_log = DataToLog {
                // TODO make Data::new() function to handle this situation.
                data: Data {
                    header: data.header.clone(),
                    rows: desired_row,
                    limit_to_show: 10
                }
            };
            return Ok(data_to_log);
        } else if by_col {

            let mut desired_col: Vec<&str> = vec![];
            data
                .header
                .iter()
                .enumerate()
                .for_each(|(i, h)| {
                    for indx in start..end {
                        if indx == i {
                            desired_col.push(h);
                        }
                    }
                });

            let mut desired_rows: Record = vec![];
            let mut hold_strs: Vec<&str> = vec![];
            for row in data.rows.iter() {
                for indx in start..end {
                    hold_strs.push(row.get(indx).unwrap())
                }
                desired_rows.push(StringRecord::from(hold_strs.clone()));
                hold_strs.clear();
            }
            let heads = StringRecord::from(desired_col);
            let data_to_log = DataToLog {
                // TODO make Data::new() function to handle this situation.
                data: Data {
                    header: heads,
                    rows: desired_rows,
                    limit_to_show: 10
                }
            };
            return Ok(data_to_log);
        } else {
            Err("".to_string())
        }

    }
    fn validate(&self, start: usize, end: usize, data: &Data) -> Result<(usize, usize), String> {
        if start > data.rows.len()
        || start > data.header.len()
        || start > end {
            Err(
                "either the `starting Range` is > rows length, `ending Range` is < rows length or the starting Range > ending Range."
                    .to_string()
            )
        } else {
            Ok((start, end))
        }
    }
}

pub fn handle_select_cmd(
    file: FileHandler,
    out: OutputForm,
    select_data: SelectBy
) {
    let data = web_or_disk_reader((file.web_file, file.disk_file))
        .expect("failed to read from web or disk be sure that you give the correct file path or url path");
    match (&select_data.row, &select_data.col) {
        (Some(row), None) => {
            let check_row = true;
            let check_col = false;
            let log = select_data.entry_handler(&row, &data, check_row, check_col);
            match log {
                Ok(l) => out.deside_output_format_and_log(l),
                Err(e) => println!("{}", e),
            }
        },
        (None, Some(col)) => {
            let check_row = false;
            let check_col = true;
            let log = select_data.entry_handler(&col, &data, check_row, check_col);
            match log {
                Ok(l) => out.deside_output_format_and_log(l),
                Err(e) => println!("{}", e),
            }
        },
        (Some(row), Some(col)) => {
            // let check_row = true;
            // let check_col = true;
            // let log = select_data;
            // match log {
            //     Ok(l) => out.deside_output_format_and_log(l),
            //     Err(e) => println!("{}", e),
            // }
        },
        (None, None) => {
        }
    };
}

