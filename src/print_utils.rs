use crate::file_utils::FileType;


pub fn print_file(p: &str, depth: usize, filetype: &FileType) {
    let mut prefix = String::new();
    for i in 0..depth {
        prefix.push_str("   ");
        if i == depth-1 {
            prefix.push('|');
        }
    }
    match filetype {
        FileType::Folder => println!("{}---> /{}", prefix, p),
        FileType::File => println!("{}---> {}", prefix, p),
        FileType::Undefined => println!("{}---> ?{}", prefix, p),
    }
}
