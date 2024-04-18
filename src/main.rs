use clap::{Parser, Subcommand};
use commands::{link, list, unlink};
use metadata::MetaData;

mod commands;
mod messages;
mod metadata;


#[derive(Clone, Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Debug, Subcommand)]
enum Commands {
    /// List servers a config is linked to
    /// Example: config-manager list CMI/config.yml
    #[clap(verbatim_doc_comment)]
    List { config: String },

    /// Link a config to the repository.
    /// Fails if there is an existing file in the repository that does not match the file we're trying to link exactly.
    /// Example: config-manager link CMI/config.yml ms
    #[clap(verbatim_doc_comment)]
    Link { config: String, server: String },

    /// Unlink a config from a server, leaving a copy of the file in place. 'all' is a valid input.
    /// If no server remains linked, the config in the repository will be deleted
    /// Example: config-manager unlink CMI/config.yml ms
    #[clap(verbatim_doc_comment)]
    Unlink { config: String, server: String },
}

fn main() {
    let cli = Cli::parse();
    let metadata = MetaData::load();
    match cli.command {
        Commands::List { config } => list(&metadata, &config),
        Commands::Link { config, server } => link(&metadata, &config, &server),
        Commands::Unlink { config, server } => unlink(&metadata, &config, &server),
    }
}
