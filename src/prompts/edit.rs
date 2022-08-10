//! Contains the execution flow for editing a secret.

use inquire::{
    formatter::MultiOptionFormatter,
    validator::{MultiOptionValidator, StringValidator},
    MultiSelect, Password, PasswordDisplayMode, Text,
};

use crate::{errors::HimitsuError, models::metadata::Anatomy};

use super::config::{self, ConfigType};

/// Run the prompts asking which secret attributes to edit.
pub fn run_edit_targets<'a>() -> Result<Vec<&'a str>, HimitsuError> {
    let render_config = config::get_inquire_config(ConfigType::Standard);

    let answer_formatter: MultiOptionFormatter<&str> = &|selections| {
        format!(
            "Updating the following attributes: {}",
            selections
                .iter()
                .map(|a| a.value.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    };
    let selector_validator: MultiOptionValidator<&str> = &|selections| {
        if selections.len() == 0 {
            Err("Please select at least one option!".to_string())
        } else {
            Ok(())
        }
    };

    let options = vec!["Category", "Label", "Secret", "Tags"];

    let mut update_targets = MultiSelect::new("Select the attributes you want to update:", options)
        .with_formatter(answer_formatter)
        .with_help_message(
            "Select at least 1 option. ↑↓ or ['j', 'k'] to move, space to select one, → to all, ← to none, type to filter",
        )
        .with_render_config(render_config)
        .with_validator(selector_validator)
        .with_vim_mode(true)
        .prompt()?;

    update_targets.sort();

    Ok(update_targets)
}

/// Run the prompt asking for a new category for this secret.
pub fn run_edit_category(new_anatomy: &mut Anatomy) -> Result<(), HimitsuError> {
    let render_config = config::get_inquire_config(ConfigType::Standard);

    let category = Text::new("Set a new category for this secret:")
        .with_help_message("(OPTIONAL) Defaults to \"Unclassified\"")
        .with_placeholder("Unclassified")
        .with_render_config(render_config)
        .prompt_skippable()?
        .unwrap_or("Unclassified".to_string());

    new_anatomy.category = category;

    Ok(())
}

/// Run the prompt asking for a new label for this secret.
pub fn run_edit_label(new_anatomy: &mut Anatomy) -> Result<(), HimitsuError> {
    let render_config = config::get_inquire_config(ConfigType::Standard);

    let label_validator: StringValidator = &|input| {
        if input.is_empty() {
            Err("A label is required!".to_string())
        } else {
            Ok(())
        }
    };

    let label = Text::new("Enter a label for this secret:")
        .with_render_config(render_config)
        .with_validator(label_validator)
        .prompt()?;

    new_anatomy.label = label;

    Ok(())
}

/// Run the prompt asking for a new secret.
pub fn run_edit_secret() -> Result<String, HimitsuError> {
    let render_config = config::get_inquire_config(ConfigType::Standard);

    let secret = Password::new("Enter your new secret:")
        .with_display_mode(PasswordDisplayMode::Hidden)
        .with_display_toggle_enabled()
        .with_render_config(render_config)
        .with_help_message("Press \"<CTRL> + r\" to reveal input.")
        .prompt()?;

    Ok(secret)
}

/// Run the prompt asking for new tags for this secret.
pub fn run_edit_tags(new_anatomy: &mut Anatomy) -> Result<(), HimitsuError> {
    let render_config = config::get_inquire_config(ConfigType::Standard);

    let tags = Text::new("Set new tags for this secret:")
        .with_help_message("(OPTIONAL) Enter a list of space-delimited tags. No default tags are applied if none are specified")
        .with_render_config(render_config)
        .prompt_skippable()?
        .unwrap_or("".to_string())
        .split(" ")
        .map(|tag| tag.to_string().to_lowercase())
        .collect();

    new_anatomy.tags = tags;

    Ok(())
}
