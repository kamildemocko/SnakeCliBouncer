use std::time::Duration;
use colored::Colorize;

pub struct Config {
    pub empty: String,
    pub bullet: String,
    pub bullet_hit: String,
    pub food: String,
    pub update_ms: Duration,
}

impl Config {
    pub fn new() -> Config {
        Config {
            empty: "-".bright_black().to_string(),
            bullet: "\u{2716}".purple().to_string(),
            bullet_hit: "\u{220C}".red().to_string(),
            food: "\u{2205}".green().to_string(),
            update_ms: Duration::from_millis(100),
        }
    }
}