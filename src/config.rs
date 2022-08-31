use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub runners: Vec<Runner>,
}

#[derive(Deserialize, Serialize)]
pub struct Runner {
    pub path: String,
    pub name: String,
}
