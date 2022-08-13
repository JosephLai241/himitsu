//! Contains the execution flow for adding a new secret.

use inquire::{self, validator::StringValidator, Password, PasswordDisplayMode, Text};

use crate::errors::HimitsuError;

use super::config::{self, ConfigType};

/// Run the add secret prompts. Ask for a label (if one is not provided), then ask for the secret
/// itself, and finally asks to set any categories or tags.
pub fn run_add_secret(
    category: &Option<String>,
    label: &Option<String>,
    tags: &Option<Vec<String>>,
) -> Result<(String, String, String, Vec<String>), HimitsuError> {
    let render_config = config::get_inquire_config(ConfigType::Standard, true);

    let label_validator: StringValidator = &|input| {
        if input.is_empty() {
            Err("A label is required!".to_string())
        } else {
            Ok(())
        }
    };

    let label_input = if label.is_some() {
        label.to_owned()
    } else {
        Text::new("Enter a label for this secret:")
            .with_render_config(render_config)
            .with_validator(label_validator)
            .prompt_skippable()?
    };
    if label_input.is_none() {
        return Err(HimitsuError::UserCancelled);
    }

    let secret_input = Password::new("Enter your secret:")
        .with_display_mode(PasswordDisplayMode::Hidden)
        .with_display_toggle_enabled()
        .with_render_config(render_config)
        .with_help_message("Press \"<CTRL> + r\" to reveal input.")
        .prompt_skippable()?;
    if secret_input.is_none() {
        return Err(HimitsuError::UserCancelled);
    }

    let category = match category {
        Some(category_name) => category_name.to_lowercase(),
        None => {
            let input = Text::new("Set a category for this secret:")
                .with_default("unclassified")
                .with_help_message("(OPTIONAL) Defaults to \"unclassified\"")
                .with_render_config(render_config)
                .prompt_skippable()?;

            if input.is_none() {
                return Err(HimitsuError::UserCancelled);
            }

            if input.as_ref().unwrap().is_empty() {
                "unclassified".to_string()
            } else {
                input.unwrap().to_lowercase()
            }
        }
    };
    let tags = match tags {
        Some(tag_values) => tag_values
            .to_owned()
            .iter()
            .map(|tag| tag.to_lowercase())
            .collect(),
        None => {
            let tags_input = Text::new("Set tags for this secret:")
                .with_help_message("(OPTIONAL) Enter a list of space-delimited tags. No default tags are applied if none are specified")
                .with_render_config(render_config)
                .prompt_skippable()?;

            if tags_input.is_none() {
                return Err(HimitsuError::UserCancelled);
            }

            tags_input
                .unwrap()
                .split(' ')
                .map(|tag| tag.to_string().to_lowercase())
                .collect::<Vec<String>>()
        }
    };

    Ok((label_input.unwrap(), secret_input.unwrap(), category, tags))
}
