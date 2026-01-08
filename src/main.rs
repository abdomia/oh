mod log;
mod reader;
mod cli;
mod data;

use crate::cli::oh::Oh;

fn main() {
    Oh::new()
        .execute()
        .handle_cmds();
}

