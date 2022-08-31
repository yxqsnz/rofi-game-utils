mod config;
mod radeontop;

use config::Config;
use rofi::Rofi;
use std::{env, fs, io};

fn main() -> io::Result<()> {
    let config_path = env::var("RGU_CONFIG_PATH")
        .unwrap_or_else(|_| String::from("~/.config/rofi-game-utils.toml"))
        .replace("~", &env::var("HOME").expect("$HOME"));
    eprintln!("Using: {config_path}");

    let content = fs::read(config_path)?;
    let config: Config = toml::from_slice(&content)?;

    radeontop::fetch_info()?;

    let options = vec![
        format!("GPU Usage: {}", radeontop::get_gpu_usage()?),
        format!("VRam Usage: {}", radeontop::get_vram_usage()?),
    ];

    Rofi::new(&options).prompt("Select").run().ok();
    Ok(())
}
