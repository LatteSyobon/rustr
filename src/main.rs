use std::fs;
use std::fs::File;
use std::path::Path;
use crate::parser::AppArg;
use clap::Parser;

mod parser;

fn main() {
    let arg: AppArg = AppArg::parse();
    let file = match File::open(Path::new(arg.file_name.as_str())) {
        Ok(f) => {
            f
        },
        Err(_) => {
            println!("error: File does not exist!");
            println!("Files existing in the current directory:");
            let paths = fs::read_dir("").unwrap();
            for path in paths {
                println!("  {}", path.unwrap().path().display())
            }
            return
        }
    };
}