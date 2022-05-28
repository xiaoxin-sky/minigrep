use std::env;
use std::process;

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("出错了：{}", error);
        process::exit(0);
    });
    match run(config) {
        Ok(_) => println!("运行结束"),
        Err(_) => process::exit(1),
    };
}
