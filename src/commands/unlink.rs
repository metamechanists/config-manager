use std::{fs, process::Command};

use crate::{messages, metadata::MetaData};

fn delete_symlink(to: &str) {
    let output = Command::new("rm")
        .arg(to)
        .output();
    if let Err(error) = output {
        panic!("{}", messages::unlink_failed(error))
    }
}

pub fn get_linked_servers(metadata: &MetaData, config: &str) -> Vec<String> {
    let mut servers = vec![];
    for server in metadata.get_servers() {
        let path = metadata.get_server_directory(&server) + "/plugins/" + config;
        if let Ok(metadata) = fs::symlink_metadata(path) {
            if metadata.is_symlink() {
                servers.push(server);
            }
        }
    }
    servers
}

pub fn unlink_config(metadata: &MetaData, config: &str, server: &str) {
    let from = metadata.get_server_directory(&server) + "/plugins/" + config;
    let to = metadata.get_config_directory() + "/" + config;

    if let Ok(metadata) = fs::symlink_metadata(&from) {
        if !metadata.is_symlink() {
            panic!("{}", messages::not_linked());
        }
    }

    delete_symlink(&from);

    let contents = fs::read(&to);
    if let Err(error) = contents {
        panic!("{}", messages::error_reading_file(to.as_str(), error))
    }

    if let Err(error) = fs::write(&from, contents.unwrap()) {
        panic!("{}", messages::error_writing_file(from.as_str(), error))
    }

    if get_linked_servers(metadata, config).is_empty() {
        if let Err(error) = fs::remove_file(&to) {
            panic!("{}", messages::error_removing_file(to.as_str(), error))
        }
    }
}