use crate::metadata::MetaData;

use self::{link::link_config, list::list_config, unlink::{get_linked_servers, unlink_config}};

mod link;
mod list;
mod unlink;

pub fn list(metadata: &MetaData, config: &str) {
    list_config(metadata, config)
}

pub fn link(metadata: &MetaData, config: &str, server: &str) {
    link_config(metadata, config, server);
}

pub fn unlink(metadata: &MetaData, config: &str, server: &str) {
    match server {
        "all" => {
            for server in get_linked_servers(metadata, config) {
                unlink_config(metadata, config, &server);
            }
        },
        _ => unlink_config(metadata, config, server),
    }
}
