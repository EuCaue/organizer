use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    times: Option<u8>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,
    Greet {
        #[arg(short, long, default_value_t = String::from("Bom dia"))]
        name: String,
    },
}

pub fn run() {
    let args = Args::parse();
    match args.command {
        Commands::Init => crate::commands::init::execute().unwrap(),
        Commands::Greet { name } => crate::commands::greet::execute(name, args.times).unwrap(),
    }
}
