//! Contains subcommands for `himitsu`.

use ansi_term::Color;
use clap::Subcommand;

use crate::{
    encryption::{
        decrypt::{self, DecryptionMode},
        encrypt,
    },
    errors::HimitsuError,
    lookup::{
        modify::{self, RemovalEvent},
        utils::{self as lookup_utils, LookupMode},
    },
    models::metadata::Anatomy,
    prompts::{add, edit, use_secret, utils},
    utils::closet,
};

/// Contains subcommands for `himitsu`.
#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// Add a new secret.
    ///
    /// NOTE: Since the `-t/--tags` flag accepts space-delimited values, this flag should be used
    /// at the end of the command.
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
        /// The label corresponding to the secret (supports Regex expressions).
        #[clap(value_parser)]
        label: Option<String>,
    },
    /// Remove an existing secret (search by label).
    Remove {
        /// The label corresponding to the secret (supports Regex expressions).
        #[clap(value_parser)]
        label: Option<String>,
    },
    /// Use a stored secret (search by label).
    Use {
        /// The label corresponding to the secret (supports Regex expressions).
        #[clap(value_parser)]
        label: Option<String>,
    },
}

/// Execution blocks for this program's subcommands.
pub fn run_subcommands(password: &str, subcommand: &SubCommands) -> Result<(), HimitsuError> {
    match subcommand {
        SubCommands::Add {
            category,
            label,
            tags,
        } => {
            let (label, secret, category, tags) = add::run_add_secret(category, label, tags)?;
            let anatomy = Anatomy::create_from(category, label, tags);

            encrypt::encrypt_secret(&anatomy, password, secret)?;
        }
        SubCommands::Edit { label } => {
            if !closet::check_for_skeletons()? {
                return Err(HimitsuError::NoSecretsError);
            }

            let label = utils::run_get_label(label)?;
            let found_matches =
                lookup_utils::search_in_lookup_table(LookupMode::Search(label), password)?;

            if found_matches.is_empty() {
                let list_all_secrets = utils::run_confirmation_prompt(
                    "No matches were found. List all stored secrets?",
                )?;

                if list_all_secrets {
                    let found_matches =
                        lookup_utils::search_in_lookup_table(LookupMode::GetAll, password)?;

                    let lookup_match = use_secret::run_select_secret(found_matches)?;
                    let secret = decrypt::decrypt_secret(
                        DecryptionMode::EditSecret,
                        &lookup_match.hash,
                        password,
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

                    modify::remove_in_lookup_table(
                        &lookup_match.hash,
                        password,
                        RemovalEvent::Replace,
                    )?;

                    encrypt::encrypt_secret(&new_anatomy, password, new_secret.unwrap_or(secret))?;
                } else {
                    println!("\n{}\n", Color::Red.bold().paint("GOODBYE."));
                }
            } else {
                let lookup_match = use_secret::run_select_secret(found_matches)?;
                let secret = decrypt::decrypt_secret(
                    DecryptionMode::EditSecret,
                    &lookup_match.hash,
                    password,
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

                modify::remove_in_lookup_table(
                    &lookup_match.hash,
                    password,
                    RemovalEvent::Replace,
                )?;

                encrypt::encrypt_secret(&new_anatomy, password, new_secret.unwrap_or(secret))?;
            }
        }
        SubCommands::Remove { label } => {
            if !closet::check_for_skeletons()? {
                return Err(HimitsuError::NoSecretsError);
            }

            let label = utils::run_get_label(label)?;
            let found_matches =
                lookup_utils::search_in_lookup_table(LookupMode::Search(label), password)?;

            if found_matches.is_empty() {
                let list_all_secrets = utils::run_confirmation_prompt(
                    "No matches were found. List all stored secrets?",
                )?;

                if list_all_secrets {
                    let found_matches =
                        lookup_utils::search_in_lookup_table(LookupMode::GetAll, password)?;

                    let lookup_match = use_secret::run_select_secret(found_matches)?;

                    if utils::run_confirmation_prompt(
                        "Are you sure you want to permanently delete the selected secret?",
                    )? {
                        modify::remove_in_lookup_table(
                            &lookup_match.hash,
                            password,
                            RemovalEvent::Remove,
                        )?;
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
                    modify::remove_in_lookup_table(
                        &lookup_match.hash,
                        password,
                        RemovalEvent::Remove,
                    )?;
                } else {
                    println!("\n{}\n", Color::Red.bold().paint("GOODBYE."));
                }
            }
        }
        SubCommands::Use { label } => {
            if !closet::check_for_skeletons()? {
                return Err(HimitsuError::NoSecretsError);
            }

            let label = utils::run_get_label(label)?;
            let found_matches =
                lookup_utils::search_in_lookup_table(LookupMode::Search(label), password)?;

            if found_matches.is_empty() {
                let list_all_secrets = utils::run_confirmation_prompt(
                    "No matches were found. List all stored secrets?",
                )?;

                if list_all_secrets {
                    let found_matches =
                        lookup_utils::search_in_lookup_table(LookupMode::GetAll, password)?;

                    let lookup_match = use_secret::run_select_secret(found_matches)?;

                    modify::update_last_accessed(&lookup_match.hash, password)?;

                    let _ = decrypt::decrypt_secret(
                        DecryptionMode::UseSecret,
                        &lookup_match.hash,
                        password,
                    )?;
                } else {
                    println!("\n{}\n", Color::Red.bold().paint("GOODBYE."));
                }
            } else {
                let lookup_match = use_secret::run_select_secret(found_matches)?;

                modify::update_last_accessed(&lookup_match.hash, password)?;

                let _ = decrypt::decrypt_secret(
                    DecryptionMode::UseSecret,
                    &lookup_match.hash,
                    password,
                )?;
            }
        }
    }

    Ok(())
}
