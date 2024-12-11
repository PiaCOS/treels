use std::fs;
use std::env;
use std::fs::metadata;
use std::path::PathBuf;


// const DEPTH: usize = 2;
const IGNORE_DOTS: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
                let depth: usize = args[1].parse().unwrap();
                treels(".", 0, depth);
            },
        3 => {
                let depth: usize = args[1].parse().unwrap();
                let dir: &str = &args[2];
                treels(dir, 0, depth)
            },
        _ => treels(".", 0, 3)
    }
}

fn is_folder(p: &PathBuf) -> bool {
    let md = metadata(p).unwrap();
    md.is_dir()
}

fn get_file_name(p: &PathBuf) -> Result<&str, &str> {
    match p.file_name().unwrap().to_str() {
        Some(file_name) => Ok(file_name),
        None => Err("non-unicode-data"),
    }
}

fn print_file(p: &str, depth: usize) {
    let mut prefix = String::new();
    for i in 0..depth {
        prefix.push_str("   ");
        if i == depth-1 {
            prefix.push_str("|");
        }
    }
    println!("{}---> {}", prefix, p);
}

fn print_dir(p: &str, depth: usize) {
    let mut prefix = String::new();
    for i in 0..depth {
        prefix.push_str("   ");
        if i == depth-1 {
            prefix.push_str("|");
        }
    }
    println!("{}---> /{}", prefix, p);
}

fn is_dot(name: &str) -> bool {
    name.chars().nth(0).unwrap() == '.'
}

fn treels(dir: &str, depth: usize, max_depth: usize) {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap()
        .map(|n| n.unwrap().path())
        .collect();
    paths.sort(); 

    for p in paths {
        // let p = path;
        let name = get_file_name(&p).unwrap();

        if !(IGNORE_DOTS && is_dot(&name)) {
            match is_folder(&p) {
                false => print_file(name, depth),
                _ => {
                    print_dir(name, depth);
                    if depth < max_depth {
                        let new_dir = p.to_str().unwrap();
                        treels(&new_dir, depth+1, max_depth);
                    }
                }
            }
        }   
    }
}

