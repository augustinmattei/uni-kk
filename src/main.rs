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
    handle_args(&args.command);
}
