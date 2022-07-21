//! Contains the execution flow for using a stored secret.

use std::collections::HashMap;

use inquire::{self, validator::StringValidator, Select, Text};

use crate::{errors::SkeletonsError, models::metadata::LookupMatch};

use super::config;

/// Run the use secret prompts:
///
/// * Ask for a label (if one is not provided)
/// * Check the lookup table for matches
///     * If there is one exact match for the label:
///         + Decrypt the secret, then copy it to the clipboard
///     * If there are multiple exact matches:
///         + Use an `inquire::Select` widget to display all matches
///         + Decrypt the secret, then copy it to the clipboard
///     * If there are no exact matches:
///         + Use the `regex` crate to search for partial label matches, then display those matches
///         in an `inquire::Select`
///         + Decrypt the secret, then copy it to the clipboard
pub fn run_use_secret(label: &Option<String>) -> Result<String, SkeletonsError> {
    inquire::set_global_render_config(config::get_inquire_config());

    let label_validator: StringValidator = &|input| {
        if input.is_empty() {
            Err("A label is required!".to_string())
        } else {
            Ok(())
        }
    };

    let label = match label {
        Some(label_value) => label_value.to_owned(),
        None => Text::new("Enter the label of the secret you want to use:")
            .with_validator(label_validator)
            .prompt()?,
    };

    Ok(label)
}

/// Run the selection prompt if multiple label matches are found in the lookup table.
pub fn run_select_secret(
    found_matches: HashMap<String, LookupMatch>,
) -> Result<String, SkeletonsError> {
    inquire::set_global_render_config(config::get_inquire_config());

    let options = found_matches
        .keys()
        .map(|painted_label| painted_label.to_string())
        .collect::<Vec<String>>();

    let selection = Select::new("Select a match:", options).prompt()?;

    match found_matches.get(&selection) {
        Some(lookup_match) => Ok(lookup_match.hash.to_string()),
        None => Err(SkeletonsError::LookupError(
            "Could not find a matching hash ID for this secret!".to_string(),
        )),
    }
}
