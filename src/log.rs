use tabled::{builder::Builder, settings::Style};
use clap::{ValueEnum};
use std::iter::once;

use crate::{data::data::Data, options::opts::{OhOptions, Points}};

#[derive(Debug, Clone)]
pub struct DataToLog<'a> {
    data: Data<'a>,
    oh_option: OhOptions,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum OutputForm {
    Table, //TODO implment this and then think of the others
    Json,
    Yaml
}

pub fn log_csv_table(data: Data) {
    let header = data.header;
    let record = data.rows;
    let mut builder = Builder::default();

    // before building we need to detect how many lines we want to render
    let lines = record.len();
    let (render_follow, limit_rows): (bool, usize);
    if lines > data.limit {
        limit_rows = data.limit;
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
