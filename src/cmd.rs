use crate::builder;
use async_std::path::PathBuf;
use async_std::{fs, future::Future, path::Path, prelude::*};
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
pub async fn build(target: Option<&str>) -> Result<(), String> {
    builder_holder(target).await;

    Ok(())
}
/// `build_holder` is a static container function that facilitates the recursive nature of 
/// [async_builder()](#cmd.async_builder)
pub fn builder_holder<'a>(target: std::option::Option<&'a str>) -> impl Future<Output = ()> + 'a {
    async_builder(target)
}

/// async_builder recursivily scans through the child documents of the provided path, calling itself if it detects a directory
/// this design is to facilitate a 1:1 directory structure copying into the **default** site directory.
async fn async_builder(target: Option<&str>) {
    let current_dir: PathBuf = match env::current_dir() {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => panic!("init failed. Current grab current directory."),
    };
    let target_dir = match target {
        Some(s) => PathBuf::from(s),
        None => current_dir.clone(),
    };

    let cwd = match current_dir.to_str() {
        Some(s) => s,
        None => ".",
    };

    let curpath = format!("{0}/_site", cwd);
    if !Path::new(&curpath).exists().await {
        fs::create_dir("_site").await.unwrap();
    }

    if let Ok(mut entries) = fs::read_dir(target_dir).await {
        while let Some(entry) = entries.next().await {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type().await {
                    if file_type.is_dir() {
                        let new_path = &entry.path();
                        if let Some(np) = new_path.to_str() {
                            build(Some(np));
                        }
                    }
                }
                let path = entry.path();
                if let Some(ext) = Path::new(&path).extension() {
                    match ext.to_str() {
                        Some(".html") | Some("html") => builder::html::html(&path, &entry).await.unwrap(),
                        Some(".md") | Some("md") => builder::markdown::markdown(&path, &entry).await.unwrap(),
                        _ => (),
                    }
                }
            }
        }
    }
}
