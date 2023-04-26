use std::fmt::Error;
mod function;

fn main() -> Result<(),Error> {
    let _ = function::run();
    return Ok(());
}