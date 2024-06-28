use std::{env, fs};
use std::path::{Path, PathBuf};
use dirs;
use serde::Deserialize;

/**
     - {DONE} read file to get paths to backup
     - {DONE} read config file
     - [TODO] exclude files
     - [TODO] create a tarball
     - [TODO] checksum to get tarballs differences
     - [TODO] push to Google Drive (?)
**/

#[derive(Deserialize)]
struct Config {
    backup: Vec<String>,
    exclude: Vec<String>
}

const CONFIG_FILE: &str = "./config.json";


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


fn path_exists_or_exit(path: Option<&String>){
    if let Some(path) = path {
        if !path_exists(path.as_str()) {
            eprintln!("Error: {} does not exists. Abort", path);
            std::process::exit(1);
        }
    } else {
        eprintln!("Error. No file path provided. Abort");
        std::process::exit(1);
    }
}


fn parse_config_file() {
    path_exists_or_exit(Some(&CONFIG_FILE.to_string()));

    let json_content= fs::read_to_string(CONFIG_FILE).unwrap();
    let json: Config = serde_json::from_str(json_content.as_str()).expect("Not a valid json");
    println!("{:?}", json.backup);
}


fn main() {


    //  ---- READ CONFIG FILE -------------------------------------------
    parse_config_file();
    std::process::exit(0);


    println!("One day, I'll be a cool backup utility :)");

    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    let mut iter = args.iter();
    let _script_path = iter.next();
    let mut file_path = None;
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-f" => {
                file_path = Some(iter.next().expect("Path not provided"));
            }
            _ => {eprintln!("{} not a valid argument", arg)}
        }
    }


    path_exists_or_exit(file_path);

    let lines = read_files(file_path.unwrap());
    let paths_to_store = real_lines(lines);
    dbg!("{:?}", paths_to_store);
}
