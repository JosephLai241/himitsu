//! Cache utilities for `skeleton`.

use std::{
    fs::{create_dir_all, File},
    io::Read,
};

use directories::ProjectDirs;
use serde_json;

use crate::{errors::SkeletonsError, models::encryption::Encryption};

/// Retrieve the stored encryption data if the `crypt.json` file exists.
pub fn get_encryption_values() -> Result<Option<Encryption>, SkeletonsError> {
    match ProjectDirs::from("", "", "skeletons") {
        Some(project_directory) => {
            let crypt_json_path = project_directory.config_dir().join("crypt.json");

            if !crypt_json_path.exists() {
                match &crypt_json_path.parent() {
                    Some(parent) => {
                        create_dir_all(parent)?;
                        let _file = File::create(&crypt_json_path)?;

                        return Ok(None);
                    }
                    None => {
                        return Err(SkeletonsError::PathError(
                            "Could not get the path to the skeletons application directory!"
                                .to_string(),
                        ))
                    }
                }
            }

            if let Ok(mut file) = File::open(&crypt_json_path) {
                let mut data = String::new();
                file.read_to_string(&mut data)?;

                Ok(Some(serde_json::from_str(&data)?))
            } else {
                // NOTE: `File::open()` returns an `Err` if the path does not exist.
                let _file = File::create(&crypt_json_path)?;

                Ok(None)
            }
        }
        None => Err(SkeletonsError::ApplicationError),
    }
}

/// Get a `File` handle to the `crypt.json` file.
pub fn get_crypt_json() -> Result<File, SkeletonsError> {
    match ProjectDirs::from("", "", "skeletons") {
        Some(project_directory) => {
            let crypt_json_path = project_directory.config_dir().join("crypt.json");

            Ok(File::create(crypt_json_path)?)
        }
        None => Err(SkeletonsError::ApplicationError),
    }
}
