use crate::{
    args::{SubCommand, SubSubCommand},
    cli::hdr::dump::dump_cart
};

pub fn handle_args(command: &Option<SubCommand>)
{
    match command {
        Some(SubCommand::Hdr {sub_command}) => {
            match sub_command {
                Some(SubSubCommand::Dump {gba_files}) => {
                    dump_cart(gba_files);
                }
                Some(SubSubCommand::Gen {binary: _, output: _, name: _}) => {
                    todo!() // args have no types to avoid warnings
                }
                &None => {
                    todo!()
                }
            }
        }
        &None => {
            todo!()
        }
    }
}
