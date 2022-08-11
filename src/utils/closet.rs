//! Contains utilities for accessing the `closet`.

use directories::ProjectDirs;

use crate::errors::HimitsuError;

/// Check if the `closet` directory contains skeletons (secrets).
pub fn check_for_skeletons() -> Result<bool, HimitsuError> {
    match ProjectDirs::from("", "", "himitsu") {
        Some(project_directory) => {
            let closet_path = project_directory.data_dir().join("closet");

            if closet_path.exists() {
                Ok(!(closet_path.read_dir()?.count() == 0))
            } else {
                Ok(false)
            }
        }
        None => Err(HimitsuError::PathError(
            "Could not get the path to the himitsu application directory!".to_string(),
        )),
    }
}
