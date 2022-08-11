//! Contains the execution flow for using a stored secret.

use std::collections::HashMap;

use ansi_term::Color;
use inquire::{self, Select};
use serde_json;

use crate::{errors::HimitsuError, models::metadata::LookupMatch};

use super::config::{self, ConfigType};

/// Run the selection prompt if multiple label matches are found in the lookup table.
pub fn run_select_secret(
    found_matches: HashMap<String, LookupMatch>,
) -> Result<LookupMatch, HimitsuError> {
    let mut pairs = HashMap::new();
    let mut options = Vec::new();

    for (painted_label, lookup_match) in found_matches.iter() {
        let option = format!(
                "{}\n      Created:       {}\n      Last accessed: {}\n      Category:      {}\n      Tags:          {}",
                painted_label,
                Color::Green
                    .bold()
                    .paint(lookup_match.anatomy.date_created.to_string()),
                Color::Cyan.bold().paint(
                    match &lookup_match.anatomy.last_accessed {
                        Some(date) => date,
                        None => "N/A",
                    }
                ),
                Color::Blue
                    .bold()
                    .paint(lookup_match.anatomy.category.to_string()),
                Color::Yellow
                    .bold()
                    .paint(if lookup_match.anatomy.tags.is_empty() {
                        "None".to_string()
                    } else {
                        format!("{:?}", lookup_match.anatomy.tags)
                    })
            );

        pairs.insert(option.to_string(), serde_json::to_string(lookup_match)?);

        options.push(option);
    }

    match Select::new("Select a match:", options)
        .with_render_config(config::get_inquire_config(ConfigType::Standard, false))
        .prompt_skippable()?
    {
        Some(selection) => match pairs.get(&selection) {
            Some(lookup_match) => Ok(serde_json::from_str(lookup_match)?),
            None => Err(HimitsuError::LookupError(
                "Could not find a matching hash ID for this secret!".to_string(),
            )),
        },
        None => Err(HimitsuError::UserCancelled),
    }
}
