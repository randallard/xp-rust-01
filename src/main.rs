#![allow(unused)]

use crate::task::Task;
use crate::prelude::*;
use std::fs::read_dir;

mod error;
mod prelude;
mod utils;
mod task;

fn main() -> Result<()> {
    let task = Task {
        done: true,
        ..Task::new("Task 0223")
    };
    println!("{task:#?}");

    Ok(())
}
