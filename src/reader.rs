use std::ffi::OsString;
use std::path::PathBuf;
use csv::Error;
use csv::StringRecord;

use crate::log::data_handler::Header;
use crate::log::data_handler::Record;
use crate::log::data_handler::Data;

pub struct OhReader {
    pub file_path: PathBuf,
    // TODO get file from internet..
    // file_web: PathWeb
}

impl OhReader {
    pub fn read_csv_file(self) -> Result<Data, Error> {
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
                    Err(_) => StringRecord::from(vec!["None"]),
                }
            })
            .collect();

        let header: Header = reader.headers()?.clone();
        // TODO make Data::new() function to handle this situation.
        Ok(Data {
            rows: rows,
            header: header,
            limit_to_show: 10
        })
    }
}

// TODO Check the destination file here.
pub fn web_or_disk_reader(file_source: (Option<OsString>, Option<PathBuf>)) -> Option<Data> {
    match file_source {
        (Some(_url), None) => {
            None
        },
        (None, Some(file_path)) => {
            let read = OhReader {
                file_path: file_path
            };
            return match read.read_csv_file() {
                Ok(data) => Some(data),
                Err(_) => None
            }
        },
        _ => None
    }
}


