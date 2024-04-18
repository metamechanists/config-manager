use std::fs;

use crate::{messages, metadata::MetaData};

pub fn list_config(metadata: &MetaData, config: &str) {
    for server in metadata.get_servers() {
        let path = metadata.get_server_directory(&server) + "/plugins/" + config;
        if let Ok(metadata) = fs::symlink_metadata(path) {
            if metadata.is_symlink() {
                println!("{}", messages::linked_list_item(&server));
            }
        }
    }
}