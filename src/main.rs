
mod tree;
mod file_utils;
mod print_utils;

use clap::Parser;
use tree::treels;

/// Display your files in a recursive manner
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Depth of the tree
    #[arg(short, long, default_value_t = 1)]
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

