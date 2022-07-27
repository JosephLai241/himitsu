//! Contains functions that modify the lookup table.

use ansi_term::Color;
use chrono::Local;
use spinners::{Spinner, Spinners};

use crate::{
    errors::HimitsuError,
    models::metadata::{Anatomy, LookupTable},
};

use super::{secure, utils};

/// Write a new secret's `Anatomy` to the lookup table. Create the file if it does not already exist.
pub fn write_to_lookup_table(
    anatomy: Anatomy,
    lookup_table: &mut LookupTable,
    password: &str,
    secret_hash: &str,
) -> Result<(), HimitsuError> {
    // TODO | FUTURE:
    //      `HashMap.insert()` returns an `Option`. A return of Some(T) indicates the value at
    //      this key was updated (overwritten).
    //      Handle table collisions in the future?
    lookup_table.table.insert(secret_hash.to_string(), anatomy);

    secure::encrypt_lookup_table(password, lookup_table)?;

    Ok(())
}

/// Update the `last_accessed` field within a secret's `Anatomy`.
pub fn update_last_accessed(hash_id: &str, password: &str) -> Result<(), HimitsuError> {
    let mut lookup_table = secure::decrypt_lookup_table(password)?;

    match lookup_table.table.get_mut(hash_id) {
        Some(mut anatomy) => {
            anatomy.last_accessed = Some(Local::now().format("%m-%d-%Y %H:%M:%S").to_string());

            secure::encrypt_lookup_table(password, &mut lookup_table)?;

            Ok(())
        }
        None => Err(HimitsuError::LookupError(
            "Could not update 'last_accessed' within this secret's Anatomy!".to_string(),
        )),
    }
}

/// Remove an existing hash and corresponding `Anatomy` in the lookup table.
pub fn remove_in_lookup_table(hash_id: &str, password: &str) -> Result<(), HimitsuError> {
    let mut removal_spinner = Spinner::new(Spinners::Aesthetic, "Removing your secret...".into());

    let mut lookup_table = secure::decrypt_lookup_table(password)?;

    match lookup_table.table.get(hash_id) {
        Some(_found_match) => {
            lookup_table.table.remove(hash_id);

            utils::remove_hash_directory(hash_id)?;

            secure::encrypt_lookup_table(password, &mut lookup_table)?;

            removal_spinner.stop_and_persist(
                "✅",
                Color::Green
                    .bold()
                    .paint("Successfully removed your secret.")
                    .to_string(),
            );

            Ok(())
        }
        None => {
            removal_spinner.stop_and_persist(
                "❗️",
                Color::Red
                    .bold()
                    .paint("SECRET REMOVAL FAILED.")
                    .to_string(),
            );

            Err(HimitsuError::LookupError(
                "Could not find an existing Anatomy for this secret!".to_string(),
            ))
        }
    }
}
