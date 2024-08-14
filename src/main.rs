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
        title: "Task01".to_string(),
        done: false,
        desc: None
    };
    println!("{task:#?}");

    Ok(())
}
