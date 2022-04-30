use async_std::{fs, path::Path};


pub async fn html<P: AsRef<Path>>(path: &P, entry: &fs::DirEntry) -> Result<(), String> {
    Ok(())
}