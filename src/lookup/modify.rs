//! Contains functions that modify the lookup table.

use chrono::Local;

use crate::{
    errors::SkeletonsError,
    models::{
        encryption::Encryption,
        metadata::{Anatomy, LookupTable},
    },
};

use super::{
    secure::{decrypt_lookup_table, encrypt_lookup_table},
    utils::remove_hash_directory,
};

/// Write a new secret's `Anatomy` to the lookup table. Create the file if it does not already exist.
pub fn write_to_lookup_table(
    anatomy: Anatomy,
    encryption_data: &Encryption,
    lookup_table: &mut LookupTable,
    secret_hash: &str,
) -> Result<(), SkeletonsError> {
    // TODO | FUTURE:
    //      `HashMap.insert()` returns an `Option`. A return of Some(T) indicates the value at
    //      this key was updated (overwritten).
    //      Handle table collisions in the future?
    lookup_table.table.insert(secret_hash.to_string(), anatomy);

    encrypt_lookup_table(encryption_data, lookup_table)?;

    Ok(())
}

/// Update the `last_accessed` field within a secret's `Anatomy`.
pub fn update_last_accessed(
    encryption_data: &Encryption,
    hash_id: &str,
) -> Result<(), SkeletonsError> {
    let mut lookup_table = decrypt_lookup_table(encryption_data)?;

    match lookup_table.table.get_mut(hash_id) {
        Some(mut anatomy) => {
            anatomy.last_accessed = Some(Local::now().format("%m-%d-%Y %H:%M:%S").to_string());

            encrypt_lookup_table(encryption_data, &mut lookup_table)?;

            Ok(())
        }
        None => Err(SkeletonsError::LookupError(
            "Could not update 'last_accessed' within this secret's Anatomy!".to_string(),
        )),
    }
}

/// Remove an existing hash and corresponding `Anatomy` in the lookup table.
pub fn remove_in_lookup_table(
    encryption_data: &Encryption,
    hash_id: &str,
) -> Result<(), SkeletonsError> {
    let mut lookup_table = decrypt_lookup_table(encryption_data)?;

    match lookup_table.table.get(hash_id) {
        Some(_found_match) => {
            lookup_table.table.remove(hash_id);

            remove_hash_directory(hash_id)?;

            encrypt_lookup_table(encryption_data, &mut lookup_table)?;

            Ok(())
        }
        None => Err(SkeletonsError::LookupError(
            "Could not find an existing Anatomy for this secret!".to_string(),
        )),
    }
}
