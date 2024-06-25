use std::{env, fs};
use std::path::{Path, PathBuf};
use dirs;

/** TODO LIST:
     - read file to get paths to backup
     - "procedural" backup (like rsync)
     - create a tarball
     - checksum to get tarballs differences
     - push to Google Drive (?)
**/

fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}


fn read_files(path: &str) -> Vec<String>{
    let mut files = Vec::new();
    for line in fs::read_to_string(path).unwrap().lines(){
        files.push(line.to_string());
    }
    files
}

fn real_lines(lines: Vec<String>) -> Vec<String>{
    let mut reals = Vec::new();
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

    for line in lines{
        let line = if line.starts_with("~"){
            line.replace("~", home_dir.to_str().unwrap())
        } else {
            line
        };
        if path_exists(line.as_str()){
            reals.push(line);
        }
    }
    reals
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
                if path_exists(file_path) {
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
    let paths_to_store = real_lines(lines);
    println!("{:?}", paths_to_store);
}
