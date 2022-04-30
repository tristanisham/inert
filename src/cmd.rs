use crate::build;
use async_std::{fs, path::Path, prelude::*};
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
pub async fn build() -> std::io::Result<()> {
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => panic!("init failed. Current grab current directory."),
    };

    fs::create_dir("_site").await?;

    let mut entries = fs::read_dir(current_dir).await?;
    while let Some(entry) = entries.next().await {
        let entry = entry?;
        let path = entry.path();
       
        if let Some(ext) = Path::new(&path).extension() {
            match ext.to_str() {
                Some(".html") | Some("html") => build::html(&path, &entry).await.unwrap(),
                Some(".md") | Some("md") => build::markdown(&path, &entry).await.unwrap(),
                _ => (),
            }
        }
        // let metadata = entry.metadata()?;
    }

    // for entry in fs::read_dir(current_dir).await {
    //     let entry = entry;
    //     let path = entry.path();
    //     // let metadata = entry.metadata()?;
    //     if let Some(ext) = Path::new(&path).extension() {
    //         match ext.to_str() {
    //             Some(".html") | Some("html") => build::html(&path).await.unwrap(),
    //             Some(".md") | Some("md") => build::markdown(&path).await.unwrap(),
    //             _ => (),
    //         }
    //     }
    // }

    Ok(())
}
