use async_std::{fs, path::Path};


pub async fn html<P: AsRef<Path>>(_path: &P, _entry: &fs::DirEntry) -> Result<(), String> {
    Ok(())
}