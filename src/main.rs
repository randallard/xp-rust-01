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
    let mut req_builder = RequestBuilder::new();
    req_builder
        .url("https://some-url.com/task/123")
        .method("GET");

    // ... do some stuff
    
    let req = req_builder   
        .header("token","user_uuid.exp.sign")
        .build()?;
    println!("{req:#?}");

    req_builder.header("Client-Version","1.2");
    let req = req_builder.build()?;
    println!("{req:#?}");
    
    Ok(())
}
