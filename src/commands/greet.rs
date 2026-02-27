use anyhow::Result;

const DEFAULT: u8 = 1;

pub fn execute(name: String, times: Option<u8>) -> Result<()> {
    let times = times.unwrap_or(DEFAULT);
    for i in DEFAULT..times + DEFAULT {
        println!("Hello, {}{} 👋", name, i);
    }
    Ok(())
}
