use crate::parser::AppArg;
use clap::Parser;

mod parser;

fn main() {
    let arg: AppArg = AppArg::parse();
    println!("{}",arg.file_name);
}