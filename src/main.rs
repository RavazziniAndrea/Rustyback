use std::{fs, fs::File, io};
use std::path::{Path, PathBuf};
use dirs;
use serde::Deserialize;
use tar::Builder;
use ignore::WalkBuilder;
use once_cell::sync::Lazy;
use flate2::Compression;
use flate2::write::GzEncoder;

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
    tarball: Tarball,
    backup: Vec<String>,
    exclude: Vec<String>
}

#[derive(Deserialize, Debug)]
struct Tarball {
    name: String,
    path: String
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


fn backup(config: Config) -> Result<(), std::io::Error>{
    let tarball = File::create(config.tarball.name)?;
    let enc = GzEncoder::new(tarball, Compression::default());
    let mut tar = tar::Builder::new(enc);
    let path = config.tarball.path;
    for f in config.backup{
        tar.append_path(f);
    }
    tar.finish();
    Ok(())
}


fn OLD_backup(config: Config) -> io::Result<()>{
    let tar = File::create("backup.tar")?;
    let mut builder = Builder::new(tar);
    for path in config.backup{
        if ! path_exists(path.as_str()){
            println!("{} doesn't exists. Skip", path);
            continue;
        }
        let mut walker = WalkBuilder::new(path);
        walker.standard_filters(false);
        let iter = walker.build();
        for result  in iter{
            if result.is_err(){continue;}
            let entry = result.unwrap();
            let path = entry.path();

            // Verifica se il file corrisponde a uno dei pattern di esclusione
            if config.exclude.iter().any(|pattern| {
                path.to_str().unwrap_or_else(||{pattern}).contains(pattern)
            }){
                println!("skippo {:?}", path);
                continue;
            }
            println!("------>>>>> {:?} -- {:?}", path, ".");
            builder.append_file(path, &mut File::open(path).unwrap()).unwrap();
        }
    }
    builder.finish()?;
    Ok(())
}

fn main() {

    println!("One day, I'll be a cool backup utility :)");

    //  ---- READ CONFIG FILE -------------------------------------------
    let config: Config = parse_config_file();
    // println!("{:?}", config);

    let res = backup(config);
    println!("{:?}", res);

}
