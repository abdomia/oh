use crate::{cli::cmds::handler::OutputForm, log::DataToLog, reader::OhReader};

pub fn handle_get_cmd(read: OhReader, out: OutputForm) -> Result<((), String), csv::Error> {
    let data = read.read_csv_file()?;
    let ready = DataToLog {
        data: data
    };
    match out {
        OutputForm::Table => Ok((ready.log_to_csv_table(), "table".to_string())),
        OutputForm::Json => Ok(((), "json here".to_string())),
        OutputForm::Yaml => Ok(((), "yaml is here".to_string()))
    }
}
