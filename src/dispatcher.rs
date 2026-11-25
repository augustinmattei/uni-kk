use anyhow::Result;

use crate::{
    args::{SubCommand, SubSubCommand},
    cli::hdr::dump::dump_cart
};

fn hdr_commands(hdr_subcommand: &Option<SubSubCommand>) -> Result<()>
{
    match hdr_subcommand {
        Some(SubSubCommand::Dump {gba_files}) => {
            return dump_cart(gba_files);
        }
        Some(SubSubCommand::Gen {binary: _, output: _, name: _}) => {
            todo!() // args have no types to avoid warnings
        }
        &None => {
            todo!()
        }
    }
}

pub fn handle_args(command: &Option<SubCommand>) -> Result<()>
{
    match command {
        Some(SubCommand::Hdr {sub_command}) => {
            return hdr_commands(sub_command);
        }
        &None => {
            todo!()
        }
    }
}
