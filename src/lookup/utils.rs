//! Contains utilities for the lookup table.

use std::{
    collections::HashMap,
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use ansi_term::{Color, Style};
use directories::ProjectDirs;
use regex::Regex;

use crate::{
    errors::SkeletonsError,
    models::{encryption::Encryption, metadata::LookupMatch},
};

use super::secure::decrypt_lookup_table;

/// Get the lookup table directory path.
pub fn get_lookup_dir_path() -> Result<PathBuf, SkeletonsError> {
    match ProjectDirs::from("", "", "skeletons") {
        Some(project_directory) => Ok(project_directory.data_dir().join("lookup")),
        None => Err(SkeletonsError::PathError(
            "Could not get the path to the skeletons application directory!".to_string(),
        )),
    }
}

/// Contains variants for lookup table search modes.
pub enum LookupMode {
    /// Get all secret data stored in the lookup table.
    GetAll,
    /// Search for a specific secret in the lookup table.
    Search(String),
}

/// Search for a label within the lookup table or return all secrets within the lookup table
/// depending on the `LookupMode`.
pub fn search_in_lookup_table(
    encryption_data: &Encryption,
    lookup_mode: LookupMode,
) -> Result<HashMap<String, LookupMatch>, SkeletonsError> {
    let lookup_table = decrypt_lookup_table(encryption_data)?;

    let mut found_matches = HashMap::new();

    match lookup_mode {
        LookupMode::GetAll => {
            for (hash_key, anatomy) in lookup_table.table.iter() {
                found_matches.insert(
                    Style::new().bold().paint(anatomy.label.clone()).to_string(),
                    LookupMatch::create(anatomy.to_owned(), hash_key.to_string()),
                );
            }
        }
        LookupMode::Search(label) => {
            let regex = Regex::new(&label)?;

            for (hash_key, anatomy) in lookup_table.table.iter() {
                if let Some(match_range) = regex.find(&anatomy.label.to_lowercase()) {
                    let mut painted_label = String::new();
                    for (index, character) in anatomy.label.chars().enumerate() {
                        if (match_range.start()..match_range.end()).contains(&index) {
                            painted_label.push_str(
                                &Color::Red.bold().paint(format!("{character}")).to_string(),
                            );
                        } else {
                            painted_label.push_str(
                                &Style::new()
                                    .bold()
                                    .paint(&format!("{character}"))
                                    .to_string(),
                            )
                        }
                    }

                    found_matches.insert(
                        painted_label,
                        LookupMatch::create(anatomy.to_owned(), hash_key.to_string()),
                    );
                }
            }
        }
    }

    Ok(found_matches)
}

/// Get the lookup table's contents from the lookup directory.
pub fn get_lookup_table() -> Result<Vec<u8>, SkeletonsError> {
    let mut lookup_file = File::open(&get_lookup_dir_path()?.join("table"))?;
    let mut lookup_table = Vec::new();
    lookup_file.read_to_end(&mut lookup_table)?;

    Ok(lookup_table)
}

/// Get the lookup table's nonce from the lookup directory.
pub fn get_lookup_nonce() -> Result<[u8; 24], SkeletonsError> {
    let mut lookup_file = File::open(&get_lookup_dir_path()?.join("nonce"))?;
    let mut lookup_nonce = [0u8; 24];
    lookup_file.read_exact(&mut lookup_nonce)?;

    Ok(lookup_nonce)
}

/// Remove the hash directory on the local machine.
pub fn remove_hash_directory(hash_id: &str) -> Result<(), SkeletonsError> {
    match ProjectDirs::from("", "", "skeletons") {
        Some(project_directory) => {
            let hash_directory = project_directory.data_dir().join("closet").join(hash_id);

            fs::remove_dir_all(hash_directory)?;

            Ok(())
        }
        None => Err(SkeletonsError::PathError(
            "Could not get the path to the skeletons application directory!".to_string(),
        )),
    }
}
