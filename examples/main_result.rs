//https://stackoverflow.com/questions/51550167/how-to-manually-return-a-result-boxdyn-error

// check exit code on bash
// https://www.cyberciti.biz/faq/linux-bash-exit-status-set-exit-statusin-bash/

// RUST_BACKTRACE=full cargo run --example main_result_3
// RUST_BACKTRACE=1 cargo run --example main_result_3

// run on bash
// cargo run --example main_result_3 && echo "success" || echo "failed"

// run on bash
// cargo run --example main_result_3 && echo "Exit code => $?"

use env_logger::Env;
use log::LevelFilter;
// use log::{debug, error, info, trace, warn};
use log::{error, info};
use std::io::Write;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}

impl Error for MyError {}

pub fn run() -> Result<(), Box<dyn Error>> {
    let condition = true;

    if condition {
        return Err(Box::new(MyError("Oops".into())));
    }

    Ok(())
}

fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let _ = env_logger::Builder::new()
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
        .try_init();

    info!("start main log");

    if let Err(e) = run() {
        // println!("{}", e); // "There is an error: Oops"
        error!("{}", e);
        error!("Exit {}", 1);
        ::std::process::exit(1);
    }
    info!("Exit {}", 0);
    ::std::process::exit(0);
}
