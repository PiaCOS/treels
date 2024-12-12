use std::fs;
use std::fs::metadata;
use std::path::PathBuf;
use clap::Parser;

/// Display your files as a tree with specific depth
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Depth of the tree
    #[arg(short, long, default_value_t = 2)]
    depth: usize,
    
    /// Path to grow your tree
    #[arg(short, long, default_value_t = String::from("."))]
    path: String,

    /// Include dotfiles in the display
    #[arg(short, long, action, default_value_t = false)]
    include_dots: bool,
}


fn main() {
    let args = Args::parse();
    treels(&args.path, 0, args.depth, args.include_dots)
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

fn treels(dir: &str, depth: usize, max_depth: usize, include_dots: bool) {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap()
        .map(|n| n.unwrap().path())
        .collect();
    paths.sort(); 

    for p in paths {
        // let p = path;
        let name = get_file_name(&p).unwrap();

        if !(!include_dots && is_dot(&name)) {
            match is_folder(&p) {
                false => print_file(name, depth),
                _ => {
                    print_dir(name, depth);
                    if depth < max_depth {
                        let new_dir = p.to_str().unwrap();
                        treels(&new_dir, depth+1, max_depth, include_dots);
                    }
                }
            }
        }   
    }
}

