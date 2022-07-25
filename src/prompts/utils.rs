//! Contains prompt utilities.

use inquire::{validator::StringValidator, Confirm, Text};

use crate::errors::SkeletonsError;

use super::config::{self, ConfigType};

/// Runs the prompt to get the label of the secret the user wants to access.
pub fn run_get_label(label: &Option<String>) -> Result<String, SkeletonsError> {
    let label_validator: StringValidator = &|input| {
        if input.is_empty() {
            Err("A label is required!".to_string())
        } else {
            Ok(())
        }
    };

    let label = match label {
        Some(label_value) => label_value.to_owned(),
        None => Text::new("Enter the label of the secret you want to access:")
            .with_render_config(config::get_inquire_config(ConfigType::Standard))
            .with_validator(label_validator)
            .prompt()?,
    };

    Ok(label)
}

/// Run a confirmation prompt with a message.
pub fn run_confirmation_prompt(message: &str) -> Result<bool, SkeletonsError> {
    Ok(
        //Confirm::new("No matches were found. List all stored secrets?")
        Confirm::new(message)
            .with_default(true)
            .with_render_config(config::get_inquire_config(ConfigType::Confirm))
            .prompt()?,
    )
}
