//! Contains the execution flow for adding a new secret.

use inquire::{self, validator::StringValidator, Password, PasswordDisplayMode, Text};

use crate::errors::SkeletonsError;

use super::config::{self, ConfigType};

/// Run the add secret prompts. Ask for a label (if one is not provided), then ask for the secret
/// itself, and finally asks to set any categories or tags.
pub fn run_add_secret(
    category: &Option<String>,
    label: &Option<String>,
    tags: &Option<Vec<String>>,
) -> Result<(String, String, String, Vec<String>), SkeletonsError> {
    let render_config = config::get_inquire_config(ConfigType::Standard);

    let label_validator: StringValidator = &|input| {
        if input.is_empty() {
            Err("A label is required!".to_string())
        } else {
            Ok(())
        }
    };

    let label = match label {
        Some(label_value) => label_value.to_owned(),
        None => Text::new("Enter a label for this secret:")
            .with_render_config(render_config)
            .with_validator(label_validator)
            .prompt()?,
    };
    let secret = Password::new("Enter your secret:")
        .with_display_mode(PasswordDisplayMode::Hidden)
        .with_display_toggle_enabled()
        .with_render_config(render_config)
        .with_help_message("Press \"<CTRL> + r\" to reveal input.")
        .prompt()?;
    let category = match category {
        Some(category_name) => category_name.to_string(),
        None => {
            let input = Text::new("Set a category for this secret:")
                .with_help_message("(OPTIONAL) Defaults to \"Unclassified\"")
                .with_placeholder("Unclassified")
                .with_render_config(render_config)
                .prompt_skippable()?
                .unwrap_or("Unclassified".to_string());

            if input.is_empty() {
                "Unclassified".to_string()
            } else {
                input
            }
        }
    };
    let tags = match tags {
        Some(tag_values) => tag_values.to_owned(),
        None => {
            let tags_input = Text::new("Set tags for this secret:")
                .with_help_message("(OPTIONAL) Enter a list of space-delimited tags. No default tags are applied if none are specified")
                .with_render_config(render_config)
                .prompt_skippable()?.unwrap_or("".to_string());

            tags_input
                .split(" ")
                .map(|tag| tag.to_string())
                .collect::<Vec<String>>()
        }
    };

    Ok((label, secret, category, tags))
}
