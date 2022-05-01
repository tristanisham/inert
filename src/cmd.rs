use crate::builder;
use async_std::{fs, path::Path, prelude::*};
use futures::future::{BoxFuture, FutureExt};
use std::env;

// pub fn init() -> Result<(), String> {
//     let current_dir = match env::current_dir() {
//         Ok(dir) => dir,
//         Err(_) => panic!("init failed. Current grab current directory."),
//     };
//     create_init_files(current_dir)
// }

// fn create_init_files(_path: PathBuf) -> Result<(), String> {
//     Ok(())
// }

/// `help()` prints out the CLI's commands, instructions on how to use each command, and other helpful information
pub fn help() {
    println!("init -> creates a pre-built application structure in the current directory");
    println!("--help ->")
}
/// `version()` prints the application's version
pub fn version() {
    println!("{0}", env!("CARGO_PKG_VERSION"))
}

/// `build()` generates inert static files in CONFIG:default_dir
///
/// *standard default dir is* `_site`
pub async fn build<'a>(target: Option<&str>) -> Result<(), String> {
    if let Some(target) = target {
        let parse_target = format!("{0}", target);
        async_builder(&parse_target).await;
    }
        
    Ok(())
}
/// `build_holder` is a static container function that facilitates the recursive nature of
/// [async_builder()](#cmd.async_builder)

/// async_builder recursivily scans through the child documents of the provided path, calling itself if it detects a directory
/// this design is to facilitate a 1:1 directory structure copying into the **default** site directory.
fn async_builder<'a>(target: &str) -> BoxFuture<()> {
    println!("{target}");
    async move {
        if let Ok(mut entries) = fs::read_dir(target).await {
            while let Some(entry) = entries.next().await {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type().await {
                        if file_type.is_dir() {
                            let new_path = &entry.path();
                            if let Some(np) = new_path.to_str() {
                                async_builder(np).await
                            }
                        }
                    }
                    let path = entry.path();
                    if let Some(ext) = Path::new(&path).extension() {
                        match ext.to_str() {
                            Some(".html") | Some("html") => {
                                builder::html::html(&path, &entry).await.unwrap()
                            }
                            Some(".md") | Some("md") => {
                                builder::markdown::markdown(&path, &entry).await.unwrap()
                            }
                            _ => (),
                        }
                    }
                }
            }
        } else {
            eprintln!("Unable to parse {target}");
        }
    }
    .boxed()
}
