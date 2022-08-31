use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub games: Vec<Game>,
}

#[derive(Deserialize, Serialize)]
pub struct Game {
    pub command: String,
    pub name: String,
}
