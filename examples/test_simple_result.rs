// FROM HERE
// https://zerotomastery.io/blog/complete-guide-to-testing-code-in-rust/

fn some_fn_ok() -> Result<bool, String> {
    Ok(true)
}

fn some_fn_err() -> Result<bool, String> {
    Err("not ok!".into())
}

fn main() {
    println!("{:?}", some_fn_ok());
    println!("{:?}", some_fn_err());
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

    // #[test]
    // #[should_panic]
    // fn panic_panics() -> ! {
    //     panic!()
    // }
}

// cargo test --example test_simple_result
