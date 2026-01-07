// FIX reverse the FILE Name and SubCommand order......
mod log;
mod reader;
mod cli;
mod data;

use crate::cli::oh::Oh;

fn main() {
    Oh::new()
        .cli()
        .handle_cmds();

}

