use AutoReagent::utility::{parameter::Args, config::Config};
use clap::Parser;

fn main(){
    let paras = Args::parse();
    let config_file = paras.config;
    let config = Config::init(&config_file.expect("No such file for config initialization"));
    println!("{:#?}",config);
}
