use csv::StringRecord;
use crate::log::DataToLog;
use crate::cli::cmds::select::OhRange;

pub type Header = StringRecord;
pub type Record = Vec<StringRecord>;

pub struct Data {
    pub header: Header,
    pub rows: Record,
    // TODO make this limit optioned by user with limitations.
    pub limit_to_show: usize
}

impl Data {
    pub fn new(
        data_header: Header,
        data_rows: Record
    ) -> Self {
        Data {
            header: data_header,
            rows: data_rows,
            limit_to_show: 10
        }
    }

    pub fn prepare_for_log_with_range(
        self,
        range_sel: Option<OhRange>,
    ) -> Result<DataToLog , String> {
        match range_sel {
            Some(rng) => {
                if rng.start > self.rows.len()
                || rng.start > rng.end {
                    Err(
                        "either the `starting Range` is > rows length, `ending Range` is < rows length or the starting Range > ending Range."
                            .to_string()
                    )
                } else {
                    let desired: Record = self
                        .rows
                        .get(rng.start..rng.end)
                        .unwrap()
                        .to_vec();

                    let data_to_log = DataToLog {
                        data: Data::new(self.header, desired)
                    };
                    return Ok(data_to_log);
                }
            },
            None => Err("range not found please specify the range that you want to select!".to_string())
        }
    }
}

