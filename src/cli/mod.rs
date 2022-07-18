//! Contains the command-line interface configuration for `skeletons`.

use clap::{Parser, Subcommand};

/// Contains all arguments used in `skeletons`.
#[derive(Debug, Parser)]
#[clap(about, author, version)]
pub struct Args {
    /// Fuck around and find out.
    #[clap(action, long)]
    pub banner: bool,

    /// Contains subcommands for `skeletons`.
    #[clap(subcommand)]
    pub subcommand: Option<SubCommands>,
}

/// Contains subcommands for `skeletons`.
#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// Add a new secret.
    Add {
        /// The name of the new secret to add.
        #[clap(value_parser)]
        name: Option<String>,
    },
    /// Edit an existing secret.
    Edit {
        /// The name of the secret to edit.
        #[clap(value_parser)]
        name: Option<String>,
    },
    /// Remove an existing secret.
    Remove {
        /// The name of the secret to remove.
        #[clap(value_parser)]
        name: Option<String>,
    },
    /// Use a stored secret.
    Use {
        /// The name of the secret to use.
        #[clap(value_parser)]
        name: Option<String>,
    },
}
