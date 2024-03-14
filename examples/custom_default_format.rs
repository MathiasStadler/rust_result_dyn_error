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

use log::info;
use log::LevelFilter;
use std::io::Write;

use env_logger::{Builder, Env};

//use env_logger::Env;

// env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

fn init_logger() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    let _from_env = &mut Builder::from_env(env);

    //env_logger::Builder::new()
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

fn main() {
    init_logger();

    info!("a log from `MyLogger`");
}
