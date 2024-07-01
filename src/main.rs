use std::{env, fs};
use std::path::{Path, PathBuf};
use dirs;
use serde::Deserialize;
use tar::Builder;
use ignore::WalkBuilder;
use once_cell::sync::Lazy;

/**
     - {DONE} read file to get paths to backup
     - {DONE} read config file
     - [TODO] exclude files
     - [TODO] create a tarball
     - [TODO] checksum to get tarballs differences
     - [TODO] push to Google Drive (?)
**/

#[derive(Deserialize, Debug)]
struct Config {
    backup: Vec<String>,
    exclude: Vec<String>
}

const CONFIG_FILE: &str = "./config.json";
static HOME_DIR: Lazy<PathBuf> = Lazy::new(|| get_home_path());

fn get_home_path() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
}

fn read_files(path: &str) -> Vec<String>{
    let mut files = Vec::new();
    for line in fs::read_to_string(path).unwrap().lines(){
        files.push(line.to_string());
    }
    files
}


fn path_exists(path: &str) -> bool {
    Path::new(&path.replace("~", HOME_DIR.to_str().unwrap())).exists()
}

fn path_exists_or_exit(path: &str){
    if !path_exists(path) {
        eprintln!("Error: {} does not exists. Abort", path);
        std::process::exit(1);
    }
}


fn parse_config_file() -> Config{
    path_exists_or_exit(CONFIG_FILE);

    let json_content= fs::read_to_string(CONFIG_FILE)
                                    .unwrap()
                                    .replace("~", HOME_DIR.to_str().unwrap());
    let json: Config = serde_json::from_str(json_content.as_str()).expect("Not a valid json");
    println!("{:?}", &json.backup);
    json
}


fn backup(config: Config){
    for path in config.backup{
        if ! path_exists(path.as_str()){
            println!("{} doesn't exists. Skip", path);
            continue;
        }
        let mut walker = WalkBuilder::new(path);
        walker.standard_filters(false);
        for exclude in &config.exclude{
            println!("{} <-----", exclude);
            walker.add_ignore(exclude);
        }
        walker.build();
        println!("{:?}", walker);
    }
}

fn main() {

    println!("One day, I'll be a cool backup utility :)");

    //  ---- READ CONFIG FILE -------------------------------------------
    let config: Config = parse_config_file();
    println!("{:?}", config);

    backup(config);

}
