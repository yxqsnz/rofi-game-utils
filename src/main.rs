mod config;
mod radeontop;
mod util;

use config::Config;
use rofi::Rofi;
use std::{env, fs, io, process::Command};

fn main() -> io::Result<()> {
    let config_path = env::var("RGU_CONFIG_PATH")
        .unwrap_or_else(|_| String::from("~/.config/rofi-game-utils.toml"))
        .replace("~", &env::var("HOME").expect("$HOME"));
    eprintln!("Using: {config_path}");

    let content = fs::read(config_path)?;
    let config: Config = toml::from_slice(&content)?;

    radeontop::fetch_info()?;

    let options = vec![
        format!("Play"),
        format!("GPU Usage: {}", radeontop::get_gpu_usage()?),
        format!("VRam Usage: {}", radeontop::get_vram_usage()?),
    ];

    if let Ok(option) = Rofi::new(&options).prompt("Select").run() {
        println!("User selected: {option}");
        if option == "Play" {
            let game_list: Vec<String> = config
                .games
                .iter()
                .map(|runner| runner.name.clone())
                .collect();

            if let Ok(runner) = Rofi::new(&game_list).prompt("Play").run() {
                println!("User wants to play: {runner}");
                let game = config
                    .games
                    .into_iter()
                    .find(|it| it.name == runner)
                    .expect("Game");
                Command::new("bash").arg("-c").arg(game.command).spawn()?;
            }
        }
    }

    Ok(())
}
