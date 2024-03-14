mod my_env_logger;
use my_env_logger::*;

fn main() {
    let _env_logger = init_logger();

    warn("warn ");
}
