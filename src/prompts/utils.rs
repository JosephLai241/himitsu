//! Contains prompt utilities.

use inquire::{validator::StringValidator, Confirm, Text};

use crate::errors::HimitsuError;

use super::config::{self, ConfigType};

/// Runs the prompt to get the label of the secret the user wants to access.
pub fn run_get_label(label: &Option<String>) -> Result<String, HimitsuError> {
    let label_validator: StringValidator = &|input| {
        if input.is_empty() {
            Err("A label is required!".to_string())
        } else {
            Ok(())
        }
    };

    match label {
        Some(label_value) => Ok(label_value.to_owned()),
        None => Text::new("Enter the label of the secret you want to access:")
            .with_help_message("Also accepts Regex expressions")
            .with_render_config(config::get_inquire_config(ConfigType::Standard, true))
            .with_validator(label_validator)
            .prompt_skippable()?
            .map_or(Err(HimitsuError::UserCancelled), Ok),
    }
}

/// Run a confirmation prompt with a message.
pub fn run_confirmation_prompt(message: &str) -> Result<bool, HimitsuError> {
    let confirmation = Confirm::new(message)
        .with_default(true)
        .with_render_config(config::get_inquire_config(ConfigType::Confirm, true))
        .prompt_skippable()?;
    if confirmation.is_none() {
        return Err(HimitsuError::UserCancelled);
    }

    Ok(confirmation.unwrap())
}
