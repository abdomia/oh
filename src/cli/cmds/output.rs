use clap::ValueEnum;
use crate::log::logger::DataToLog;

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputForm {
    Table, // this is the default
    Json,
    Yaml
}

impl OutputForm {
    pub fn deside_output_format_and_log(&self, ready_data: DataToLog) {
        match self {
            OutputForm::Table => {
                ready_data.log_to_csv_table();
                println!("table");
            },
            OutputForm::Json => println!("json"),
            OutputForm::Yaml => println!("yaml is here")
        }
    }
}

