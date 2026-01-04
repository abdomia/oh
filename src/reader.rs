use anyhow::{Result, Error};
use csv::{self, StringRecord};
use std::ffi::OsString;

pub fn read_csv(file: OsString) -> Result<(StringRecord, Vec<StringRecord>)> {
    let r = csv::Reader::from_path(file);
    let mut reader = match r {
        Ok(v) => v,
        Err(e) => return Err(Error::new(e))
    };

    let mut rows: Vec<StringRecord> = vec![];
    for vs in reader.records() {
        if let Ok(v) = vs {
            rows.push(v);
        }
    };

    let headers = reader.headers();
    return match headers {
        Ok(h) => {
            Ok((h.clone(), rows))
        },
        Err(e) => Err(Error::new(e))
    };
}

