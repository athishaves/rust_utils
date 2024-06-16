use std::fs;
use std::io;

pub fn delete_file(file_path: &str) -> io::Result<()> {
    fs::remove_file(file_path)?;
    Ok(())
}
