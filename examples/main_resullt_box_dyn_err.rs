// FROM HERE
// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html


mod my_env_logger;

#[allow(unused_imports)]
use my_env_logger::*;

// fn main() {
//     let _env_logger = init_logger();

//     warn("warn ");
// }

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let result: Result<(), Box<dyn Error>> = Ok(());
    result?;
    Ok(())
}

// cargo run --example main_resullt_box_dyn_err
