//! Contains subcommands for `skeletons`.

use clap::Subcommand;

use crate::{
    encryption::encrypt::encrypt_secret, errors::SkeletonsError, models::encryption::Encryption,
    utils::anatomy,
};

/// Contains subcommands for `skeletons`.
#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// Add a new secret.
    Add {
        /// Set a category for this secret.
        #[clap(long, short)]
        category: Option<String>,

        /// The secret itself.
        #[clap(value_parser)]
        secret: Option<String>,

        /// Set tags for this secret. Enter multiple values delimited by a space to set multiple
        /// tags.
        #[clap(long, multiple_values = true, short)]
        tags: Option<Vec<String>>,
    },
    /// Edit an existing secret (search by label).
    Edit {
        /// The label corresponding to the secret.
        #[clap(value_parser)]
        label: Option<String>,
    },
    /// Remove an existing secret (search by label).
    Remove {
        /// The label corresponding to the secret.
        #[clap(value_parser)]
        label: Option<String>,
    },
    /// Use a stored secret (search by label).
    Use {
        /// The label corresponding to the secret.
        #[clap(value_parser)]
        label: Option<String>,
    },
}

/// Execution block for this program's subcommands.
pub fn run_subcommands(
    encryption_data: &Encryption,
    subcommand: &SubCommands,
) -> Result<(), SkeletonsError> {
    match subcommand {
        SubCommands::Add {
            category,
            secret,
            tags,
        } => match secret {
            Some(secret_value) => {
                let anatomy = anatomy::create_new_anatomy(category, tags);
                encrypt_secret(&anatomy, encryption_data, &secret_value)?;
            }
            None => {}
        },
        SubCommands::Edit { label } => {}
        SubCommands::Remove { label } => {}
        SubCommands::Use { label } => {}
    }

    Ok(())
}
