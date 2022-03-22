use std::env;
use std::fs;
use std::panic;

fn main() {
    let args: Vec<String> = env::args().collect(); 
    let path = &args[1];

    let dir = fs::read_dir(path);

    let dir = match dir {
        Ok(dir) => dir,
        Err(error) => panic!("ERROR: {}", error),
    };

    for file in dir {
        println!("{}", file.unwrap().path().display());
    }
}
