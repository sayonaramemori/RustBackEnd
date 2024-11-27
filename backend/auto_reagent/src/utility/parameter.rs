use clap::Parser;
use std::path::PathBuf;

/// The Backend implemented by Rust
#[derive(Debug,Parser)]
#[command(about, long_about=None)]
pub struct Args{
    /// Config all in a file
    #[arg(short,long,value_name = "FILE.yml")]
    pub config: Option<PathBuf>,
}
