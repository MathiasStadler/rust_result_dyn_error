// FROM HERE
// https://stackoverflow.com/questions/61810740/log-source-file-and-line-numbers


//https://github.com/rust-cli/env_logger/tree/main/examples
use env_logger::Env;
use log::{info, LevelFilter};
use std::io::Write;

fn main() {


    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    env_logger::Builder::new()
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

    info!("hello world")
}