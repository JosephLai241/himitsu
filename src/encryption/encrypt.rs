//! Contains encryption functions for `skeletons`.

use ansi_term::Color;
use chacha20poly1305::{
    aead::{consts::U24, generic_array::GenericArray, Aead, NewAead},
    Key, XChaCha20Poly1305, XNonce,
};
use data_encoding::HEXLOWER;
use rand::{rngs::OsRng, RngCore};
use ring::digest::{Context, SHA256};
use spinners::{Spinner, Spinners};

use crate::{
    authentication::get_argon2_config,
    errors::SkeletonsError,
    lookup::{decrypt_lookup_table, write_to_lookup_table},
    models::{encryption::Encryption, metadata::Anatomy},
    utils::store,
};

/// Store the encrypted secret.
pub fn encrypt_secret(
    anatomy: &Anatomy,
    encryption_data: &Encryption,
    secret: String,
) -> Result<(), SkeletonsError> {
    let mut encryption_spinner =
        Spinner::new(Spinners::Aesthetic, "Encrypting your secret...".into());

    let mut secret_salt = [0u8; 32];
    let mut secret_nonce = [0u8; 24];
    OsRng.fill_bytes(&mut secret_salt);
    OsRng.fill_bytes(&mut secret_nonce);

    // Generate a new hash for this particular secret.
    let argon2_config = get_argon2_config();
    let key = argon2::hash_raw(&encryption_data.password_hash, &secret_salt, &argon2_config)?;

    let cipher = XChaCha20Poly1305::new(Key::from_slice(&key));

    let nonce = XNonce::from_slice(&secret_nonce);

    match cipher.encrypt(nonce, secret.as_bytes()) {
        Ok(ciphertext) => {
            encryption_spinner.stop_and_persist(
                "✅",
                Color::Green
                    .bold()
                    .paint("Successfully encrypted your secret.")
                    .to_string(),
            );

            update_lookup_table(anatomy, ciphertext, encryption_data, nonce)?;

            Ok(())
        }
        Err(error) => {
            encryption_spinner.stop_and_persist(
                "❗️",
                Color::Red
                    .bold()
                    .paint("SECRET ENCRYPTION FAILED.")
                    .to_string(),
            );

            Err(SkeletonsError::AEADEncryptionError(error.to_string()))
        }
    }
}

/// Generate a SHA256 hash for a new secret.
fn generate_sha256_hash(
    anatomy: &Anatomy,
    ciphertext: &Vec<u8>,
    nonce: &GenericArray<u8, U24>,
) -> String {
    let mut hash_string = String::from_utf8_lossy(&ciphertext).to_string();
    hash_string.push_str(&format!(
        "{}{}{}{:?}{:?}",
        anatomy.category, anatomy.date_created, anatomy.label, anatomy.last_accessed, anatomy.tags
    ));
    hash_string.push_str(&String::from_utf8_lossy(&nonce));

    let mut context = Context::new(&SHA256);
    context.update(hash_string.as_bytes());

    HEXLOWER.encode(context.finish().as_ref())
}

/// Update the lookup table with the secret's hash ID and anatomy.
fn update_lookup_table(
    anatomy: &Anatomy,
    ciphertext: Vec<u8>,
    encryption_data: &Encryption,
    nonce: &GenericArray<u8, U24>,
) -> Result<(), SkeletonsError> {
    let mut write_spinner = Spinner::new(Spinners::Noise, "Storing your secret...".into());

    let secret_hash = generate_sha256_hash(&anatomy, &ciphertext, nonce);

    let mut lookup_table = decrypt_lookup_table(&encryption_data)?;

    write_to_lookup_table(
        anatomy.to_owned(),
        encryption_data,
        &mut lookup_table,
        &secret_hash,
    )?;

    store::store_secret(ciphertext, nonce, &secret_hash)?;

    write_spinner.stop_and_persist(
        "🔒",
        Color::Green
            .bold()
            .paint("Secret has been stored!")
            .to_string(),
    );

    Ok(())
}
