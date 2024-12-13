use std::fs::metadata;
use std::path::{PathBuf, Path};


pub enum FileType {
    Folder,
    File,
    Undefined,
}

pub fn is_folder(p: &PathBuf) -> Option<bool> {
    match metadata(p)  {
        Ok(md) => Some(md.is_dir()),
        Err(_) => None,
    }
}

pub fn determine_file_type(p: &PathBuf) -> FileType {
    match is_folder(p) {
        Some(true) => FileType::Folder,
        Some(false) => FileType::File,
        None => FileType::Undefined,
    }
}

pub fn get_file_name(p: &Path) -> Result<&str, &str> {
    match p.file_name().unwrap().to_str() {
        Some(file_name) => Ok(file_name),
        None => Err("non-unicode-data"),
    }
}

pub fn is_dot(name: &str) -> bool {
    name.starts_with('.')
}