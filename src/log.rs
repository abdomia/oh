use tabled::{builder::Builder, settings::Style};
use std::iter::once;

use crate::data::data_handler::Data;

pub struct DataToLog {
    pub data: Data,
}

impl DataToLog {
    pub fn log_to_csv_table(&self) {
        let header = &self.data.header;
        let record = &self.data.rows;
        let mut builder = Builder::default();

        // before building we need to detect how many lines we want to render
        let lines = record.len();
        let (render_follow, limit_rows): (bool, usize);
        if lines > self.data.limit_to_show {
            limit_rows = self.data.limit_to_show;
            render_follow = true;
        } else {
            limit_rows = lines;
            render_follow = false;
        }

        // building the header first
        builder.push_record(header.iter());

        // building then the records
        for i in 0..limit_rows {
            let lines = record.iter()
                .nth(i)
                .unwrap();
                // .unwrap(); // for now!!!

            builder.push_record(lines.iter())
        }

        builder.insert_column(0,
            once(String::new())
                .chain(
                    (0..limit_rows)
                        .map(|x| x.to_string())
                ));
        if render_follow {
            builder.push_record((0..=header.len()).map(|_| {
                format!("...")
            }));
        };

        let mut table = builder.build();
        table.with(Style::rounded());
        println!("{}", table);
    }
}

