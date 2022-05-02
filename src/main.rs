use async_std::task;
use std::env;

mod builder;
mod cmd;
mod serve;

fn main() {
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
        if i == 1 {
            match args[1].as_str() {
                // "init" => cmd::init().unwrap(),
                "build" | "b" => {
                    if args.len() >= 3 {
                        task::block_on(cmd::build(Some(&args[2]))).unwrap();
                        return;
                    }
                    task::block_on(cmd::build(None)).unwrap();
                }
                "init" => task::block_on(cmd::init()).unwrap(),
                "install" => {
                    if args.len() >= 3 {
                        if let Err(e) = task::block_on(cmd::install(&args[2])) {
                            eprintln!("{e}")
                        }
                        return;
                    }
                    eprintln!("Please provide 1 argument. Typically a package's name");
                },
                "serve" => task::block_on(cmd::serve(8080)).unwrap(),
                "--help" => cmd::help(),
                "-v" => cmd::version(),
                _ => cmd::help(),
            };
        }
    }
}
