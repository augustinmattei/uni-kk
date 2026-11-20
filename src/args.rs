use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum SubSubCommand {
    /// Display cartridge header information
    #[command(about, long_about = "Display cartridge header information")]
    Dump {
        gba_files: String
    },

    /// Header generation
    #[command(about, long_about = "Header generation")]
    Gen {
        /// input binary file [required]
        #[arg(short, long)]
        binary: String,

        /// output filename [required]
        #[arg(short, long)]
        output: String,

        /// internal cartridge name (default: filename)
        #[arg(short, long, default_missing_value = "filename")]
        name: Option<String>
    }
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// Cartridge header interface
    Hdr {
        #[command(subcommand)]
        sub_command: SubSubCommand,
    }
}



#[derive(Parser)]
#[command(version, about, long_about = "Cartridge SDK CLI entry")]
pub struct Args {
    /// Command to execute
    #[command(subcommand)]
    command: SubCommand
}
