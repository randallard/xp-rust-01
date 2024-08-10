#![allow(unused)]

use crate::prelude::*;
use std::fs::read_dir;

mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    println!("Hello, world!");

    for entry in read_dir("./")?.filter_map(|e| e.ok()){
        let entry = entry
            .path()
            .to_str()
            .map(String::from)
            .ok_or_else( || {
                Error::Generic(f!("Invalid path {entry:?}"))
            })?;
        println!("{entry:?}");
    }

    Ok(())
}
