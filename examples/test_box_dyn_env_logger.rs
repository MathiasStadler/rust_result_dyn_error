// FROM HERE
// https://stackoverflow.com/questions/58393250/returning-error-message-to-function-expecting-boxdyn-error

use std::error::Error;
use std::fmt;

mod my_env_logger;
// use my_env_logger::*;

#[allow(unused_imports)]
use env_logger::{Builder, Env};
#[allow(unused_imports)]
use log::LevelFilter;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
#[allow(unused_imports)]
use std::io::Write;

pub fn init_logger() {
    const LOG_STYLE: &str = r#"auto"#;
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", LOG_STYLE);

    let _from_env = &mut Builder::from_env(env);

    _from_env
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(Some("logger_example"), LevelFilter::Debug)
        .init();
}

#[derive(Debug)]
struct StrError<'a>(&'a str);

// Error doesn't require you to implement any methods, but
// your type must also implement Debug and Display.
impl<'a> Error for StrError<'a> {}

impl<'a> fmt::Display for StrError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to the Display impl for `&str`:
        self.0.fmt(f)
    }
}

#[allow(dead_code)]
fn foo_result_ok() -> Result<String, Box<dyn Error>> {
    Ok("Ok ...foo_result_ok ".to_string())
}

fn foo_result_err() -> Result<String, Box<dyn Error>> {
    Err(Box::new(StrError("Error ...foo_result_err")))
}

// #[allow(dead_code)]
// fn test_case() {
//     init_logger();
//     error!("a log got a error");
//     warn!(" a log got a warning");
//     info!(" a log got a info");
//     debug!("a log got a debug");
//     trace!("a log got a trace");
// }

#[allow(dead_code)]
fn test_env_logger() {
    my_env_logger::init_logger();
    // init_logger();
    my_env_logger::error("a log got a error");
    my_env_logger::warn(" a log got a warning");
    my_env_logger::info(" a log got a info");
    my_env_logger::debug("a log got a debug");
    my_env_logger::trace("a log got a trace");
}

fn main() {
    // test_case();
    test_env_logger();

    #[allow(unused_variables)]
    let foo_result_ok: Result<String, Box<dyn Error>> = foo_result_ok();

    #[allow(unused_variables)]
    match foo_result_ok {
        Ok(result) => println!("Ok => {:?}", result),
        Err(error) => panic!("Err => {:?}", error),
    };

    #[allow(unused_variables)]
    let foo_result_err: Result<String, Box<dyn Error>> = foo_result_err();

    #[allow(unused_variables)]
    match foo_result_err {
        Ok(result) => println!(" {:?}", result),
        Err(error) => panic!("Err => {:?}", error),
    };
}

// test

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn result_test_ok() -> Result<(), Box<dyn Error>> {
        #[allow(unused_variables)]
        let foo_result_ok: Result<String, Box<dyn Error>> = foo_result_ok();

        #[allow(unused_variables)]
        let result: () = match foo_result_ok {
            Ok(result) => println!("Ok => {:?}", result),
            Err(error) => panic!("Err => {:?}", error),
        };

        Ok(result)
    }

    #[test]
    fn result_test_err() -> Result<(), Box<dyn Error>> {
        #[allow(unused_variables)]
        let foo_result_err: Result<String, Box<dyn Error>> = foo_result_err();

        #[allow(unused_variables)]
        let result: () = match foo_result_err {
            Ok(result) => println!(" {:?}", result),
            Err(error) => panic!("Err => {:?}", error),
        };

        Ok(result)
    }
}

// cargo run --example test_box_dyn_err_1
// cargo test --example test_box_dyn_err_1
