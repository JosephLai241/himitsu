//! Contains subcommands for `skeletons`.

use ansi_term::Color;
use clap::Subcommand;

use crate::{
    encryption::{
        decrypt::{self, DecryptionMode},
        encrypt,
    },
    errors::SkeletonsError,
    lookup::{self, LookupMode},
    models::{encryption::Encryption, metadata::Anatomy},
    prompts::{add, edit, use_secret, utils},
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

/// Execution blocks for this program's subcommands.
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
            let anatomy = Anatomy::create_from(category, label, tags);

            encrypt::encrypt_secret(&anatomy, encryption_data, secret)?;
        }
        SubCommands::Edit { label } => {
            let label = utils::run_get_label(label)?;
            let found_matches =
                lookup::search_in_lookup_table(encryption_data, LookupMode::Search(label))?;

            if found_matches.is_empty() {
                let list_all_secrets = utils::run_confirmation_prompt(
                    "No matches were found. List all stored secrets?",
                )?;

                if list_all_secrets {
                    let found_matches =
                        lookup::search_in_lookup_table(encryption_data, LookupMode::GetAll)?;

                    let lookup_match = use_secret::run_select_secret(found_matches)?;
                    let secret = decrypt::decrypt_secret(
                        DecryptionMode::EditSecret,
                        encryption_data,
                        &lookup_match.hash,
                    )?
                    .unwrap();

                    let update_targets = edit::run_edit_targets()?;

                    let mut new_anatomy = lookup_match.anatomy.clone();
                    let mut new_secret = None;

                    for target in update_targets {
                        match target {
                            "Category" => edit::run_edit_category(&mut new_anatomy)?,
                            "Label" => edit::run_edit_label(&mut new_anatomy)?,
                            "Secret" => new_secret = Some(edit::run_edit_secret()?),
                            "Tags" => edit::run_edit_tags(&mut new_anatomy)?,
                            _ => {}
                        }
                    }

                    lookup::remove_in_lookup_table(encryption_data, &lookup_match.hash)?;

                    encrypt::encrypt_secret(
                        &new_anatomy,
                        encryption_data,
                        new_secret.unwrap_or(secret),
                    )?;
                } else {
                    println!("\n{}\n", Color::Red.bold().paint("GOODBYE."));
                }
            } else {
                let lookup_match = use_secret::run_select_secret(found_matches)?;
                let secret = decrypt::decrypt_secret(
                    DecryptionMode::EditSecret,
                    encryption_data,
                    &lookup_match.hash,
                )?
                .unwrap();

                let update_targets = edit::run_edit_targets()?;

                let mut new_anatomy = lookup_match.anatomy.clone();
                let mut new_secret = None;

                for target in update_targets {
                    match target {
                        "Category" => edit::run_edit_category(&mut new_anatomy)?,
                        "Label" => edit::run_edit_label(&mut new_anatomy)?,
                        "Secret" => new_secret = Some(edit::run_edit_secret()?),
                        "Tags" => edit::run_edit_tags(&mut new_anatomy)?,
                        _ => {}
                    }
                }

                lookup::remove_in_lookup_table(encryption_data, &lookup_match.hash)?;

                encrypt::encrypt_secret(
                    &new_anatomy,
                    encryption_data,
                    new_secret.unwrap_or(secret),
                )?;
            }
        }
        SubCommands::Remove { label } => {
            let label = utils::run_get_label(label)?;
            let found_matches =
                lookup::search_in_lookup_table(encryption_data, LookupMode::Search(label))?;

            if found_matches.is_empty() {
                let list_all_secrets = utils::run_confirmation_prompt(
                    "No matches were found. List all stored secrets?",
                )?;

                if list_all_secrets {
                    let found_matches =
                        lookup::search_in_lookup_table(encryption_data, LookupMode::GetAll)?;

                    let lookup_match = use_secret::run_select_secret(found_matches)?;

                    if utils::run_confirmation_prompt(
                        "Are you sure you want to permanently delete the selected secret?",
                    )? {
                        lookup::remove_in_lookup_table(encryption_data, &lookup_match.hash)?;
                    } else {
                        println!("\n{}\n", Color::Red.bold().paint("GOODBYE."));
                    }
                } else {
                    println!("\n{}\n", Color::Red.bold().paint("GOODBYE."));
                }
            } else {
                let lookup_match = use_secret::run_select_secret(found_matches)?;

                if utils::run_confirmation_prompt(
                    "Are you sure you want to permanently delete the selected secret?",
                )? {
                    lookup::remove_in_lookup_table(encryption_data, &lookup_match.hash)?;
                } else {
                    println!("\n{}\n", Color::Red.bold().paint("GOODBYE."));
                }
            }
        }
        SubCommands::Use { label } => {
            let label = utils::run_get_label(label)?;
            let found_matches =
                lookup::search_in_lookup_table(encryption_data, LookupMode::Search(label))?;

            if found_matches.is_empty() {
                let list_all_secrets = utils::run_confirmation_prompt(
                    "No matches were found. List all stored secrets?",
                )?;

                if list_all_secrets {
                    let found_matches =
                        lookup::search_in_lookup_table(encryption_data, LookupMode::GetAll)?;

                    let lookup_match = use_secret::run_select_secret(found_matches)?;

                    lookup::update_last_accessed(encryption_data, &lookup_match.hash)?;

                    let _ = decrypt::decrypt_secret(
                        DecryptionMode::UseSecret,
                        encryption_data,
                        &lookup_match.hash,
                    )?;
                } else {
                    println!("\n{}\n", Color::Red.bold().paint("GOODBYE."));
                }
            } else {
                let lookup_match = use_secret::run_select_secret(found_matches)?;

                lookup::update_last_accessed(encryption_data, &lookup_match.hash)?;

                let _ = decrypt::decrypt_secret(
                    DecryptionMode::UseSecret,
                    encryption_data,
                    &lookup_match.hash,
                )?;
            }
        }
    }

    Ok(())
}
