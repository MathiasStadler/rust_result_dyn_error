// FROM HERE
// https://rust-cli.github.io/book/tutorial/errors.html
// https://rust-cli.github.io/book/tutorial/errors.html

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let result = std::fs::read_to_string("test.txt");
//     let content = match result {
//         Ok(content) => content,
//         Err(error) => {
//             return Err(error.into());
//         }
//     };
//     println!("file content: {}", content);
//     Ok(())
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = std::fs::read_to_string("test.txt");
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            panic!("Can't deal with {}, just exit here", error);
        }
    };
    println!("file content: {}", content);
    Ok(())
}
// cargo run --package rust_result_dyn_error --example main_result_box_dyn_err_1
