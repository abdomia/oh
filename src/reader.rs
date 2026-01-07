use csv::Error;
use csv::StringRecord;
use std::path::PathBuf;

use crate::cli::oh::Header;
use crate::cli::oh::Record;

pub struct OhReader {
    pub file_path: PathBuf,
    pub source: ReaderSource,
    // TODO get file from internet..
    // file_web: PathWeb
}

pub enum ReaderSource {
    Disk,
    Web,
}

impl OhReader {
    fn read_csv_from_disk(self) -> Result<(
        Header, // header
        Record // rows
    ), Error> {
        let r = csv::Reader::from_path(self.file_path.into_os_string());
        let mut reader = match r {
            Ok(v) => v,
            Err(e) => return Err(e)
        };

        let rows: Record = reader
            .records()
            .map(|e| {
                match e {
                    Ok(val) => val,
                    Err(_) => StringRecord::from(vec!["NoData"]),
                }
            })
            .collect();

        let header: Header = reader.headers()?.clone();
        Ok((header, rows))
    }
}

