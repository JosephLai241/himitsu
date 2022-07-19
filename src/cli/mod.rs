//! Contains the command-line interface configuration for `skeletons`.

pub mod subcommands;

use subcommands::SubCommands;

use clap::Parser;

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
