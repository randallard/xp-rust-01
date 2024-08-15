// region:    --- Modules

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>; // For early dev.

mod fs;

use crate::fs::list_files;

// endregion: --- Modules



fn main() -> Result<()> {

    let files = list_files(".")?;
    println!("{files:#?}");

    Ok(())
}


