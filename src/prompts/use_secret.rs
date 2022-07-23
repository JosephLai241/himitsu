//! Contains the execution flow for using a stored secret.

use std::collections::HashMap;

use ansi_term::{Color, Style};
use inquire::{self, validator::StringValidator, Confirm, Select, Text};

use crate::{errors::SkeletonsError, models::metadata::LookupMatch};

use super::config::{self, ConfigType};

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
            .with_render_config(config::get_inquire_config(ConfigType::Standard))
            .with_validator(label_validator)
            .prompt()?,
    };

    Ok(label)
}

/// Run the selection prompt if multiple label matches are found in the lookup table.
pub fn run_select_secret(
    found_matches: HashMap<String, LookupMatch>,
) -> Result<String, SkeletonsError> {
    let mut pairs = HashMap::new();
    let mut options = Vec::new();

    for (painted_label, lookup_match) in found_matches.iter() {
        let option = format!(
                "{}\n      Created:       {}\n      Last accessed: {}\n      Category:      {}\n      Tags:          {}",
                Style::new().bold().paint(painted_label.to_string()),
                Color::Green
                    .bold()
                    .paint(format!("{}", lookup_match.anatomy.date_created)),
                Color::Cyan.bold().paint(format!(
                    "{}",
                    match &lookup_match.anatomy.last_accessed {
                        Some(date) => date,
                        None => "N/A",
                    }
                )),
                Color::Blue
                    .bold()
                    .paint(format!("{}", lookup_match.anatomy.category)),
                Color::Yellow
                    .bold()
                    .paint(if lookup_match.anatomy.tags.is_empty() {
                        "None".to_string()
                    } else {
                        format!("{:?}", lookup_match.anatomy.tags)
                    })
            );

        pairs.insert(option.to_string(), lookup_match.hash.to_string());
        options.push(option);
    }

    let selection = Select::new("Select a match:", options)
        .with_render_config(config::get_inquire_config(ConfigType::Standard))
        .prompt()?;

    match pairs.get(&selection) {
        Some(hash) => Ok(hash.to_string()),
        None => Err(SkeletonsError::LookupError(
            "Could not find a matching hash ID for this secret!".to_string(),
        )),
    }
}

/// Run the selection prompt to list all stored secrets if no matches are found and the user
/// consents to listing all stored secrets.
pub fn run_show_all_secrets() -> Result<bool, SkeletonsError> {
    Ok(
        Confirm::new("No matches were found. List all stored secrets?")
            .with_default(true)
            .with_render_config(config::get_inquire_config(ConfigType::Confirm))
            .prompt()?,
    )
}
