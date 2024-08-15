// region:    --- Modules

mod error;
mod fs;

pub use self::error::{Error, Result};

use crate::fs::list_files;

// endregion: --- Modules



fn main() -> Result<()> {

    let files = list_files(".")?;
    println!("{files:#?}");

    Ok(())
}


