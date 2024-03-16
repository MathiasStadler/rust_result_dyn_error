// FROM HERE
// https://stackoverflow.com/questions/58393250/returning-error-message-to-function-expecting-boxdyn-error

use std::error::Error;
use std::fmt;

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
    Ok("Ok ...".to_string())
}

fn foo_result_err() -> Result<String, Box<dyn Error>> {
    Err(Box::new(StrError("Error...")))
}

fn main() {
    #[allow(unused_variables)]
    let foo_result_ok: Result<String, Box<dyn Error>> = foo_result_ok();

    #[allow(unused_variables)]
    let result: () = match foo_result_ok {
        Ok(result) => println!("Ok => {:?}", result),
        Err(error) => panic!("Err => {:?}", error),
    };

    #[allow(unused_variables)]
    let foo_result_err: Result<String, Box<dyn Error>> = foo_result_err();

    #[allow(unused_variables)]
    let result: () = match foo_result_err {
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
