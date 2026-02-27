use bomdia::services::config::{Config, Root};

fn main() -> anyhow::Result<()> {
    let config = Config::load().unwrap_or_else(|err| {
        eprintln!("Error while loading config. {}", err);
        std::process::exit(1);
    });
    println!("CONFIG: {:#?}", config);
    for roots in &config.root {
        let Root { folders, path } = roots;
        println!("Root folder: {:#?}", folders);
        println!("Root path: {:#?}", path);
    }
    println!("CONFIG: {:#?}", Config::path());
    Ok(bomdia::run())
}
