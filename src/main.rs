use std::env;
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


fn main() {
    println!("One day, I'll be a cool backup utility :)");

    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    let mut iter = args.iter();
    let script_path = iter.next();
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
}
