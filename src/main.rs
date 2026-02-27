use bomdia::services::config::get_config;

fn main() -> anyhow::Result<()> {
    let config = get_config();
    println!("CONFIG: {:#?}", config);
    Ok(bomdia::run())
}
