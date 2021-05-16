use std::{fs, path::PathBuf};
use toml::Value;

#[derive(Debug)]
pub struct Config {
    reports_path: PathBuf,
    date_format: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut reports_path = home::home_dir().expect("Impossible to locate $HOME dir");
        reports_path.push(".timetracer-reports");

        Self {
            reports_path,
            date_format: "%Y-%m-%d][%H:%M:%S".to_string(),
        }
    }
}

pub fn read() -> Config {
    let mut cfg = Config::default();

    if let Some(mut path) = home::home_dir() {
        path.push(".timetracerc.toml");

        println!("toml path: {}", path.display());

        if let Ok(contents) = fs::read_to_string(path) {
            println!(".timetracerc contents = {}", contents);
            match contents.parse::<Value>() {
                Ok(value) => {
                    if let Some(date_format) = value["date_format"].as_str() {
                        cfg.date_format = date_format.to_string();
                    }
                }
                Err(e) => {
                    eprintln!("ivalid toml format. {}", e);
                }
            }
        }
    }

    cfg
}
