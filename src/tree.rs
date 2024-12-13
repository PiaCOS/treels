use std::fs;
use std::path::PathBuf;

use crate::print_utils::print_file;
use crate::file_utils::{
    get_file_name,
    is_dot,
    determine_file_type,
    FileType,
};


pub fn treels(dir: &str, depth: usize, max_depth: usize, include_dots: bool) {
    let mut paths: Vec<PathBuf> = match fs::read_dir(dir) {
        Ok(read_dir) => read_dir
            .filter_map(|entry| entry.ok().map(|n| n.path()))
            .collect(),
        Err(_) => Vec::new(),
    };
    paths.sort(); 

    for p in paths {
        // let p = path;
        let name = get_file_name(&p).unwrap();
        
        if include_dots || !is_dot(name) {
            let filetype = determine_file_type(&p);
            match filetype {
                FileType::Folder => {
                    print_file(name, depth, &filetype) ;
                    if depth < max_depth {
                        let new_dir = p.to_str().unwrap();
                        treels(new_dir, depth+1, max_depth, include_dots);
                    }
                }
                _ => print_file(name, depth, &filetype),
            }
        }   
    }
}