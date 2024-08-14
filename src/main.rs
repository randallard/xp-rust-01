#![allow(unused)]

use prelude::*;
use task::Task;
use web::{RequestBuilder, Request};

mod error;
mod prelude;
mod utils;
mod task;
mod web;

fn main() -> Result<()> {
    let req = RequestBuilder::new()
        .url("https://some-url.com/task/123")
        .method("GET")
        .header("token","user_uuid.exp.sign")
        .header("some_header_name","some_header_value")
        .build()?;
    println!("{req:#?}");
    Ok(())
}
