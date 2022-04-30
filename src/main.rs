use async_std::task;
use std::env;

mod builder;
mod cmd;

fn main() {
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
        if i == 1 {
            match args[1].as_str() {
                // "init" => cmd::init().unwrap(),
                "build" => {
                    if args.len() >= 3 {
                        task::block_on(cmd::build(Some(&args[2]))).unwrap();
                    }
                    task::block_on(cmd::build(None)).unwrap();
                }
                "--help" => cmd::help(),
                "-v" => cmd::version(),
                _ => cmd::help(),
            };
        }
    }
}
