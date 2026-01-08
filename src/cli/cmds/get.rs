use crate::cli::cmds::handler::FileHandler;
use crate::cli::cmds::output::OutputForm;
use crate::log::logger::DataToLog;
use crate::reader::web_or_disk_reader;

pub fn handle_get_cmd(file: FileHandler, out: OutputForm) {
    let data = web_or_disk_reader((file.web_file, file.disk_file))
        .expect("failed to read from web or disk be sure that you give the correct file path or url path");
    let ready = DataToLog {
        data: data
    };
    out.deside_output_format_and_log(ready);
}

