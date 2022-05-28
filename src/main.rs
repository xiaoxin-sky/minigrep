use std::env;
use std::process;

use minigrep::{run, Config};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|error| {
        eprintln!("出错了：{}", error);
        process::exit(0);
    });
    if let Err(e) = run(config) {
        eprintln!("运行出错{}", e);
        process::exit(0);
    }
}
