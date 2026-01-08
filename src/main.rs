mod reader;
mod cli;
mod log;

use crate::cli::oh::Oh;

fn main() {
    Oh::new()
        .execute()
        .handle_cmds();
}

