// FROM HERE
// https://raw.githubusercontent.com/rust-cli/env_logger/main/examples/custom_default_format.rs

/*!
Disabling parts of the default format.

Before running this example, try setting the `MY_LOG_LEVEL` environment variable to `info`:

```no_run,shell
$ export MY_LOG_LEVEL='info'
```

Also try setting the `MY_LOG_STYLE` environment variable to `never` to disable colors
or `auto` to enable them:

```no_run,shell
$ export MY_LOG_STYLE=never
```

If you want to control the logging output completely, see the `custom_logger` example.
*/

use log::LevelFilter;
use log::{debug, error, info, trace, warn};
use std::io::Write;

use env_logger::{Builder, Env};

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

#[allow(dead_code)]
fn test_case() {
    init_logger();
    error!("a log got a error");
    warn!(" a log got a warning");
    info!(" a log got a info");
    debug!("a log got a debug");
    trace!("a log got a trace");
}

#[allow(dead_code)]
fn main() {
    test_case();
    // empty
}

#[allow(dead_code)]
pub fn error(log_entry: &str) -> (){
    error!("{}", log_entry)
}

#[allow(dead_code)]
pub fn warn(log_entry: &str) {
    warn!("{}", log_entry);
}

#[allow(dead_code)]
pub fn info(log_entry: &str) {
    info!("{}", log_entry);
}

#[allow(dead_code)]
pub fn debug(log_entry: &str) {
    debug!("{}", log_entry);
}

#[allow(dead_code)]
pub fn trace(log_entry: &str) {
    trace!("{}", log_entry);
}

//test
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;

    #[test]
    fn test_add() {
        assert_eq!("error", super::error("error"));
    }
}

/*
cargo run --example custom_default_format
cargo fmt
MY_LOG_LEVEL='trace' cargo run --example my_env_logger
*/
