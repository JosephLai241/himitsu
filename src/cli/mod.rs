//! Contains the command-line interface configuration for `himitsu`.

pub mod subcommands;

use subcommands::SubCommands;

use clap::Parser;

/// Contains all arguments used in `himitsu`.
#[derive(Debug, Parser)]
#[clap(about, author, version)]
pub struct Args {
    /// Fuck around and find out.
    #[clap(action, long)]
    pub banner: bool,

    /// Contains subcommands for `himitsu`.
    #[clap(subcommand)]
    pub subcommand: Option<SubCommands>,
}

#[cfg(test)]
pub mod test_top_level_cli {
    use assert_cmd::Command;

    /// Test an invalid argument passed into the application.
    #[test]
    fn test_invalid_arg() {
        Command::cargo_bin("hmu")
            .unwrap()
            .arg("-q")
            .assert()
            .failure();
    }
}
