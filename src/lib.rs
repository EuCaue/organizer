pub mod cli;
pub mod commands;
pub mod services;

use anyhow::Result;

pub fn run() {
    println!("Hello from lib.");
    cli::run()
}
