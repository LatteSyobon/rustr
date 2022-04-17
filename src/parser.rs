use clap::Parser;

#[derive(Parser)]
#[clap(
name = "rustr",
author = "LatteSyobon",
version = "v1.0.0",
about = "Rust transpiler"
)]
pub struct AppArg {
    #[clap(short, long)]
    pub lang: Option<String>,

    pub file_name: String,
}