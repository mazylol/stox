use std::{fs, fs::File, io::Write};

use directories::ProjectDirs;

use crate::types::{Config, Save};

impl Config {
    pub fn load() -> Self {
        let path = ProjectDirs::from("com", "mazylol", "stox")
            .unwrap()
            .config_dir()
            .join("config.json");
        let file = File::open(&path);
        if file.is_err() {
            fs::create_dir_all(&path.parent().unwrap()).unwrap();
            let mut file = File::create(&path).unwrap();
            let config = Config {
                api_key: String::from("REPLACE_ME"),
            };
            let json = serde_json::to_string_pretty(&config).unwrap();
            file.write_all(json.as_bytes()).unwrap();
            return config;
        } else {
            let config: Config = serde_json::from_reader(file.unwrap()).unwrap();

            if config.api_key == "REPLACE_ME" {
                panic!("API Key needs set at {:?}", path);
            }

            return config;
        }
    }
}

impl Save {
    pub fn load() -> Self {
        let path = ProjectDirs::from("com", "mazylol", "stox")
            .unwrap()
            .config_dir()
            .join("save.json");
        let file = File::open(&path);
        if file.is_err() {
            fs::create_dir_all(&path.parent().unwrap()).unwrap();
            let mut file = File::create(&path).unwrap();
            let save = Save {
                stocks: Vec::new(),
                balance: 0.0,
            };
            let json = serde_json::to_string_pretty(&save).unwrap();
            file.write_all(json.as_bytes()).unwrap();
            return save;
        } else {
            let save: Save = serde_json::from_reader(file.unwrap()).unwrap();
            return save;
        }
    }

    pub fn save(&self) {
        let path = ProjectDirs::from("com", "mazylol", "stox")
            .unwrap()
            .config_dir()
            .join("save.json");
        let mut file = File::create(&path).unwrap();
        let json = serde_json::to_string_pretty(&self).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let config = Config::load();
        println!("{}", config.api_key);
    }
}
