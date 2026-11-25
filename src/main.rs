mod args;
mod dispatcher;
mod cli;

use clap::{Parser};

use crate::{
    args::{Args},
    dispatcher::handle_args,
};

fn main() {
    let args = &Args::parse();

    match handle_args(&args.command) {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
