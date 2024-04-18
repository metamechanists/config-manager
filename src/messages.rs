use std::io;

use colored::Colorize;

pub fn cannot_link_directory() -> String {
    format!("{}", "You cannot link a directory!".bright_red())
}

pub fn already_linked() -> String {
    format!("{}", "Config already linked!".bright_red())
}

pub fn not_linked() -> String {
    format!("{}", "Config is already not linked!".bright_red())
}

pub fn not_identical() -> String {
    format!("{}", "New config is not identical to the existing config!".bright_red())
}

pub fn could_not_find_file(file: &str) -> String {
    format!("{}{}", "Could not find ".bright_red(), file.bright_cyan())
}

pub fn error_reading_file(file: &str, error: io::Error) -> String {
    format!("{}{}{}{}", " Error reading ".bright_red(), file.bright_cyan(), ": ".bright_red(), error)
}

pub fn error_writing_file(file: &str, error: io::Error) -> String {
    format!("{}{}{}{}", " Error writing ".bright_red(), file.bright_cyan(), ": ".bright_red(), error)
}

pub fn error_removing_file(file: &str, error: io::Error) -> String {
    format!("{}{}{}{}", " Error removing ".bright_red(), file.bright_cyan(), ": ".bright_red(), error)
}

pub fn failed_to_parse_json(error: serde_json::Error) -> String {
    format!("{}{}", "Failed to parse JSON: ".bright_red(), error)
}

pub fn server_does_not_exist(server: &str) -> String {
    format!("{}{}{}", "Server '".bright_red(), server, "' does not exist".bright_red())
}

pub fn link_failed(error: io::Error) -> String {
    format!("{}{}", "Failed to link: ".bright_red(), error)
}

pub fn unlink_failed(error: io::Error) -> String {
    format!("{}{}", "Failed to unlink: ".bright_red(), error)
}

pub fn linked_list_item(server: &str) -> String {
    format!("{}{}", "- ".bright_green(), server.bright_cyan())
}