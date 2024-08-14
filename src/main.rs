#![allow(unused)]

use crate::task::Task;
use crate::prelude::*;
use std::fs::read_dir;

mod error;
mod prelude;
mod utils;
mod task;

fn main() -> Result<()> {
    let task: Option<Task> = None;
    let task = task.unwrap_or_default();
    println!("{task:#?}");

    Ok(())
}
