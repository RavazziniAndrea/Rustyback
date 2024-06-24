use std::{env, fs};
use std::path::Path;

/** TODO LIST:
     - read file to get paths to backup
     - "procedural" backup (like rsync)
     - create a tarball
     - checksum to get tarballs differences
     - push to Google Drive (?)
**/

fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}


fn read_files(path: &str) -> Vec<String>{
    let mut files = Vec::new();
    for line in fs::read_to_string(path).unwrap().lines(){
        files.push(line.to_string());
    }
    files
}

fn main() {
    println!("One day, I'll be a cool backup utility :)");

    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    let mut iter = args.iter();
    let _script_path = iter.next();
    let mut file_path = "";
    while let Some(arg) = iter.next(){
        match arg.as_str() {
            "-f" => {
                file_path = iter.next().expect("Path not provided");
                if file_exists(file_path) {
                    println!("EXIXSTE")
                } else {
                    panic!("{} does not exists. Abort", file_path);
                }
            }
            _ => {println!("{} not a valid argument", arg)}
        }
    }

    if file_path.is_empty(){
        panic!("No filepath provided.")
    }
    let lines = read_files(file_path);
    println!("{:?}", lines);
}
