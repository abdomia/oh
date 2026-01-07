use std::path::PathBuf;

use crate::{log::DataToLog, reader::{OhReader, ReaderSource}};

pub fn handle_get_cmd(read: OhReader) {
    let data = read.read_csv_file().unwrap();
    let ready = DataToLog {
        data: data
    };
    ready.log_to_csv_table();
}
