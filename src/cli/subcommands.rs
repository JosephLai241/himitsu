//! Contains subcommands for `skeletons`.

use ansi_term::Color;
use clap::Subcommand;

use crate::{
    encryption::{self, decrypt, encrypt},
    errors::SkeletonsError,
    lookup::{self, LookupMode},
    models::encryption::Encryption,
    prompts::{
        add,
        use_secret::{self},
    },
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

        /// The secret's label.
        #[clap(value_parser)]
        label: Option<String>,

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
            label,
            tags,
        } => {
            let (label, secret, category, tags) = add::run_add_secret(category, label, tags)?;
            let anatomy = anatomy::create_new_anatomy(category, label, tags);

            encrypt::encrypt_secret(&anatomy, encryption_data, secret)?;
        }
        SubCommands::Edit { label } => {}
        SubCommands::Remove { label } => {}
        SubCommands::Use { label } => {
            let label = use_secret::run_use_secret(label)?;
            let found_matches =
                lookup::search_in_lookup_table(encryption_data, LookupMode::Search(label))?;

            if found_matches.is_empty() {
                let list_all_secrets = use_secret::run_show_all_secrets()?;

                if list_all_secrets {
                    let found_matches =
                        lookup::search_in_lookup_table(encryption_data, LookupMode::GetAll)?;
                    let hash_id = use_secret::run_select_secret(found_matches)?;

                    lookup::update_last_accessed(encryption_data, &hash_id)?;

                    decrypt::decrypt_secret(encryption_data, &hash_id)?;
                } else {
                    println!("\n{}\n", Color::Red.bold().paint("GOODBYE."));
                }
            } else {
                let hash_id = use_secret::run_select_secret(found_matches)?;

                lookup::update_last_accessed(encryption_data, &hash_id)?;

                decrypt::decrypt_secret(encryption_data, &hash_id)?;
            }
        }
    }

    Ok(())
}
