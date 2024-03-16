// FROM HERE
// https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/

fn some_fn_ok() -> Result<bool, String> {
    Ok(true)
}

fn some_fn_err() -> Result<bool, String> {
    Err("not ok!".into())
}

fn main() -> Result<(), String> {
    println!("{:?}", some_fn_ok());
    println!("{:?}", some_fn_err());
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn result_test_ok() -> Result<(), String> {
        // We can use question mark instead of unwrap.
        // If some_fn() is `Err`, then the test will
        // fail at this line.

        let is_ok = some_fn_ok()?;

        if is_ok {
            Ok(())
        } else {
            // `Err` fails the test
            Err("not ok!".into())
        }
    }

    #[test]
    fn result_test_err() -> Result<(), String> {
        // We can use question mark instead of unwrap.
        // If some_fn() is `Err`, then the test will
        // fail at this line.

        let is_ok = some_fn_err()?;

        if is_ok {
            Ok(())
        } else {
            // `Err` fails the test
            Err(r#"see the err => not ok!"#.into())
        }
    }

    #[test]
    fn result_test_main() -> () {
        let result_main: Result<(), String> = Ok(main().expect("REASON"));

        #[allow(unused_variables)]
        let result: () = match result_main {
            Ok(result) => println!("Ok => {:?}", result),
            Err(error) => panic!("Err => {:?}", error),
        };
    }

    // #[test]
    // #[should_panic]
    // fn panic_panics() -> ! {
    //     panic!()
    // }
}

// cargo test --example test_simple_result_main
