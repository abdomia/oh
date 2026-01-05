use csv::StringRecord;

use crate::options::opts::{OhOptions, Points};
use crate::log::DataToLog;
use crate::log::OutputForm;

#[derive(Debug, Clone)]
pub struct Data<'a> {
    pub header: &'a StringRecord,
    pub rows: &'a Vec<StringRecord>,
    // TODO make this limit optioned by user with limitations.
    pub limit: usize
}

impl<'a> Data<'a> {
    pub fn new(
        data_header: &'a StringRecord,
        data_rows: &'a Vec<StringRecord>
    ) -> Self {
        Data {
            header: data_header,
            rows: data_rows,
            limit: 10
        }
    }

    pub fn prepare_for_log_with_range(
        &self,
        range_sel: Points,
        oh_options: OhOptions
    ) -> Result<Output, String> {
    }
}
