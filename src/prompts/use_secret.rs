//! Contains the execution flow for using a stored secret.

use std::collections::HashMap;

use ansi_term::{Color, Style};
use inquire::{self, Select};
use serde_json;

use crate::{errors::SkeletonsError, models::metadata::LookupMatch};

use super::config::{self, ConfigType};

/// Run the selection prompt if multiple label matches are found in the lookup table.
pub fn run_select_secret(
    found_matches: HashMap<String, LookupMatch>,
) -> Result<LookupMatch, SkeletonsError> {
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

        pairs.insert(option.to_string(), serde_json::to_string(lookup_match)?);

        options.push(option);
    }

    let selection = Select::new("Select a match:", options)
        .with_render_config(config::get_inquire_config(ConfigType::Standard))
        .prompt()?;

    match pairs.get(&selection) {
        Some(lookup_match) => Ok(serde_json::from_str(lookup_match)?),
        None => Err(SkeletonsError::LookupError(
            "Could not find a matching hash ID for this secret!".to_string(),
        )),
    }
}
