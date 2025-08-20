mod cli;
mod commands;
mod store;

use cli::{Cli};
use clap::Parser;
use commands::handle_command;


fn main(){
    // println!("Budget management system initialized.");
     let cli = Cli::parse();
    // let cli = Cli::parse();
    handle_command(cli.command);
}
