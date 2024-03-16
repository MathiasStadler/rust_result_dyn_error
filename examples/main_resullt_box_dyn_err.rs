// FROM HERE
// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html

// https://tourofrust.com/99_en.html

mod my_env_logger;

#[allow(unused_imports)]
use my_env_logger::*;

// fn main() {
//     let _env_logger = init_logger();

//     warn("warn ");
// }

use std::error::Error;

// fn main() -> Result<(), Box<dyn Error>> {
//     let result: Result<(), Box<dyn Error>> = Ok(());
//     result?;
//     Ok(())
// }

use std::fmt::Display;
// use std::error::Error;

struct Pie;

#[derive(Debug)]
struct NotFreshError;

impl Display for NotFreshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "This pie is not fresh!")
    }
}

impl Error for NotFreshError {}

impl Pie {
    fn eat(&self) -> Result<(), Box<dyn Error>> {
        Err(Box::new(NotFreshError))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let heap_pie = Box::new(Pie);
    heap_pie.eat()?;
    Ok(())
}

// cargo run --example main_resullt_box_dyn_err
