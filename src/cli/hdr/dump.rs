use std::path::Path;

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum DumpError {
    #[error("uni-kk hdr dump : File not found.")]
    FileNotFound,
}

pub fn dump_cart(filename: &String) -> Result<()>
{
    let file_path = Path::new(filename);

    if file_path.exists() == false {
        return Err(DumpError::FileNotFound.into());
    }

    return Ok(());
}
