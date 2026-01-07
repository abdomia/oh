use crate::cli::cmds::handler::OhCommands;
use crate::log::DataToLog;
use crate::cli::oh::Header;
use crate::cli::oh::Record;
use crate::cli::oh::OhRange;

pub struct Data<'a> {
    pub header: &'a Header,
    pub rows: Record,
    // TODO make this limit optioned by user with limitations.
    pub limit_to_show: usize
}

impl<'a> Data<'a> {
    pub fn new(
        data_header: &'a Header,
        data_rows: Record
    ) -> Self {
        Data {
            header: data_header,
            rows: data_rows,
            limit_to_show: 10
        }
    }

    pub fn prepare_for_log_with_range(
        &self,
        range_sel: Option<OhRange>,
        // make it optional for now.
    ) -> Result<DataToLog<'a> , String> {
        match range_sel {
            Some(rng) => {
                if rng.start > self.rows.len()
                || rng.start > rng.end {
                    Err(
                        "either the `starting Range` is > rows length, `ending Range` is < rows length or the starting Range > ending Range. "
                            .to_string()
                    )
                } else {
                    let desired: Record = self
                        .rows
                        .get(rng.start..rng.end)
                        .unwrap()
                        .to_vec();

                    let data_logger = DataToLog {
                        data: Data::new(&self.header, desired),
                        //FIX temperoary data
                        oh_option: OhCommands::Temp
                    };
                    return Ok(data_logger);
                }
            },
            None => Err("range not found please specify the range that you want to select!".to_string())
        }
    }
}

