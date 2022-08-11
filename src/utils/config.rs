//! Config utilities for `himitsu`.

use std::{
    fs::{self, File},
    io::Read,
};

use directories::ProjectDirs;
use serde_json;

use crate::{errors::HimitsuError, models::encryption::Encryption};

/// Retrieve the stored encryption data if the `crypt.json` file exists.
pub fn get_encryption_values() -> Result<Option<Encryption>, HimitsuError> {
    match ProjectDirs::from("", "", "himitsu") {
        Some(project_directory) => {
            let crypt_json_path = project_directory.config_dir().join("crypt.json");

            if !crypt_json_path.exists() {
                Ok(None)
            } else {
                if let Ok(mut file) = File::open(&crypt_json_path) {
                    let mut data = String::new();
                    file.read_to_string(&mut data)?;

                    Ok(Some(serde_json::from_str(&data)?))
                } else {
                    Ok(None)
                }
            }
        }
        None => Err(HimitsuError::ApplicationError),
    }
}

/// Get a `File` handle to the `crypt.json` file.
pub fn get_crypt_json() -> Result<File, HimitsuError> {
    match ProjectDirs::from("", "", "himitsu") {
        Some(project_directory) => {
            let crypt_json_path = project_directory.config_dir().join("crypt.json");

            if !crypt_json_path.exists() {
                match &crypt_json_path.parent() {
                    Some(parent) => {
                        fs::create_dir_all(parent)?;
                        let _file = File::create(&crypt_json_path)?;
                    }
                    None => {
                        return Err(HimitsuError::PathError(
                            "Could not get the path to the himitsu application directory!"
                                .to_string(),
                        ));
                    }
                }
            }

            Ok(File::create(crypt_json_path)?)
        }
        None => Err(HimitsuError::ApplicationError),
    }
}
