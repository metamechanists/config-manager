use std::{collections::HashMap, fs::File, io::Read};

use serde::Deserialize;
use serde_json::from_str;

use crate::messages;

const METADATA_PATH: &str = "/home/idra/GitHub/config-manager/metadata.json";

#[derive(Deserialize)]
pub struct MetaData {
    configs: String,
    servers: HashMap<String, String>,
}

impl MetaData {
    pub fn load() -> Self {
        let file = File::open(METADATA_PATH);
        let Ok(mut file) = file else {
            panic!("{}", messages::could_not_find_file(METADATA_PATH));
        };

        let mut json_string = String::new();
        if let Err(error) = file.read_to_string(&mut json_string) {
            panic!("{}", messages::error_reading_file(METADATA_PATH, error));
        }

        match from_str(json_string.as_str()) {
            Err(error) => panic!("{}", messages::failed_to_parse_json(error)),
            Ok(paths) => paths,
        }
    }

    pub fn get_config_directory(&self) -> String {
        self.configs.clone()
    }

    pub fn get_server_directory(&self, server: &str) -> String {
        self.servers.get(server)
            .unwrap_or_else(|| panic!("{}", messages::server_does_not_exist(server).as_str()))
            .to_string()
    }

    pub fn get_servers(&self) -> Vec<String> {
        self.servers.keys().cloned().collect()
    }
}