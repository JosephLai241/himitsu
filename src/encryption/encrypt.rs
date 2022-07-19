//! Contains encryption functions for `skeletons`.

use ansi_term::Color;
use chacha20poly1305::{
    aead::{Aead, NewAead},
    Key, XChaCha20Poly1305, XNonce,
};
use rand::{rngs::OsRng, RngCore};
use spinners::{Spinner, Spinners};

use crate::{
    authentication::get_argon2_config, errors::SkeletonsError, models::encryption::Encryption,
    utils::store,
};

/// Store the encrypted secret.
pub fn encrypt_secret(
    encryption_data: &Encryption,
    secret_value: &str,
) -> Result<(), SkeletonsError> {
    let mut encryption_spinner =
        Spinner::new(Spinners::Aesthetic, "Encrypting your secret...".into());

    let mut secret_salt = [0u8; 32];
    let mut secret_nonce = [0u8; 24];
    OsRng.fill_bytes(&mut secret_salt);
    OsRng.fill_bytes(&mut secret_nonce);

    // Generate a new hash for this particular secret.
    let argon2_config = get_argon2_config();
    let key = argon2::hash_raw(
        &encryption_data.password_hash.as_bytes(),
        &secret_salt,
        &argon2_config,
    )?;

    let cipher = XChaCha20Poly1305::new(Key::from_slice(&key));

    let nonce = XNonce::from_slice(&secret_nonce);

    match cipher.encrypt(XNonce::from_slice(&secret_nonce), secret_value.as_bytes()) {
        Ok(ciphertext) => {
            encryption_spinner
                .stop_and_persist("💯", format!("{}", Color::Green.bold().paint("Done.")));

            let mut write_spinner = Spinner::new(Spinners::Noise, "Storing your secret...".into());
            store::store_secret(ciphertext, nonce)?;
            write_spinner.stop_and_persist("🔒", "Secret has been stored!".into());
        }
        Err(error) => {
            encryption_spinner.stop_and_persist("❗️", "ENCRYPTION FAILED.".into());
            return Err(SkeletonsError::AEADEncryptionError(error.to_string()));
        }
    }

    Ok(())
}
