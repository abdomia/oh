use std::ffi::OsString;
use std::path::PathBuf;
use csv::Error;
use csv::StringRecord;

use crate::data::data_handler::Header;
use crate::data::data_handler::Record;
use crate::data::data_handler::Data;

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
        Ok(Data::new(header, rows))
    }
}

// TODO Check the destination file here.
pub fn read_from_file_source(file_source: (Option<OsString>, Option<PathBuf>)) -> Result<OhReader, ()> {
    match file_source {
        (Some(_url), None) => {
            Err(()) // error for now
        },
        (None, Some(file_path)) => {
            Ok(OhReader {
                file_path: file_path
            })
        },
        _ => Err(())
    }
}


