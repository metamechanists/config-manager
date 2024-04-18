use std::{fs, path::Path, process::Command};

use crate::{messages, metadata::MetaData};

fn create_symlink(from: &str, to: &str) {
    let output = Command::new("ln")
        .arg("-s")
        .arg(to)
        .arg(from)
        .output();
    if let Err(error) = output {
        panic!("{}", messages::link_failed(error))
    }
}

fn is_identical(file_1: &str, file_2: &str) -> bool {
    let contents_1 = fs::read_to_string(file_1);
    if let Err(error) = contents_1 {
        panic!("{}", messages::error_reading_file(file_1, error));
    }

    let contents_2 = fs::read_to_string(file_2);
    if let Err(error) = contents_2 {
        panic!("{}", messages::error_reading_file(file_1, error));
    }

    contents_1.ok() == contents_2.ok()
}

pub fn link_config(metadata: &MetaData, config: &str, server: &str) {
    let from = metadata.get_server_directory(server) + "/plugins/" + config;
    let to = metadata.get_config_directory() + "/" + config;

    if Path::is_dir(Path::new(&from)) {
        panic!("{}", messages::cannot_link_directory());
    }

    if Path::is_symlink(Path::new(&from)) {
        panic!("{}", messages::already_linked());
    }

    // Config already exists in repository = check that they're identical
    // Config doesn't exist = copy 'from' to repository
    if Path::exists(Path::new(&to)) {
        if !is_identical(from.as_str(), to.as_str()) {
            panic!("{}", messages::not_identical());
        }

    } else {
        let path = std::path::Path::new(&to);
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();

        let contents = fs::read(&from);
        if let Err(error) = contents {
            panic!("{}", messages::error_reading_file(from.as_str(), error))
        }

        if let Err(error) = fs::write(&to, contents.unwrap()) {
            panic!("{}", messages::error_writing_file(to.as_str(), error))
        }
    }

    if let Err(error) = fs::remove_file(&from) {
        panic!("{}", messages::error_removing_file(from.as_str(), error))
    }

    create_symlink(&from, &to);
}
