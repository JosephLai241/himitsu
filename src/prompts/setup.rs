//! Contains the execution flow for the initial setup prompts.

use std::{fs, io::Write};

use ansi_term::Color;
use chacha20poly1305::{
    aead::{Aead, NewAead},
    Key, XChaCha20Poly1305, XNonce,
};
use inquire::{self, validator::StringValidator, Password, PasswordDisplayMode};
use rand::{self, rngs::OsRng, RngCore};
use serde_json;
use spinners::{Spinner, Spinners};

use crate::{
    authentication,
    errors::HimitsuError,
    lookup::utils,
    models::{encryption::Encryption, metadata::LookupTable},
    utils::config,
};

use super::config::{get_inquire_config, ConfigType};

/// Run the initial setup's prompt - set a master password to unlock the vault.
pub fn run_initial_setup_prompts() -> Result<Encryption, HimitsuError> {
    let password_validator: StringValidator = &|input| {
        if input.chars().count() < 10 {
            Err("The password must have at least 10 characters!".to_string())
        } else {
            Ok(())
        }
    };

    let password = Password::new("Set a password for your vault:")
        .with_display_mode(PasswordDisplayMode::Hidden)
        .with_display_toggle_enabled()
        .with_help_message(
            "Password must have at least 10 characters. Press \"<CTRL> + r\" to reveal input",
        )
        .with_render_config(get_inquire_config(ConfigType::Standard))
        .with_validator(password_validator)
        .prompt()?;

    let mut loading_bar = Spinner::new(Spinners::Aesthetic, "Generating encryption data...".into());

    let encryption_data = generate_salt_and_password_hash(&password)?;
    create_lookup_table_and_nonce(&encryption_data)?;

    let mut crypt_json = config::get_crypt_json()?;
    crypt_json.write_all(serde_json::to_string(&encryption_data)?.as_bytes())?;

    loading_bar.stop_and_persist(
        "✅",
        Color::Green
            .bold()
            .paint("Vault is configured.")
            .to_string(),
    );

    Ok(encryption_data)
}

/// Generate a new salt and password hash.
fn generate_salt_and_password_hash(password: &str) -> Result<Encryption, HimitsuError> {
    let mut salt = [0u8; 32];
    OsRng.fill_bytes(&mut salt);

    Ok(Encryption {
        password_hash: authentication::generate_raw_hash(password, &salt)?,
        salt,
    })
}

/// Create a new lookup table and nonce, then write the two values to a file in the `lookup`
/// directory.
fn create_lookup_table_and_nonce(encryption_data: &Encryption) -> Result<(), HimitsuError> {
    let lookup_dir_path = utils::get_lookup_dir_path()?;

    let cipher = XChaCha20Poly1305::new(Key::from_slice(&encryption_data.password_hash));

    let mut nonce = [0u8; 24];
    OsRng.fill_bytes(&mut nonce);

    let lookup_nonce = XNonce::from_slice(&nonce);

    let lookup_table = LookupTable::new();

    match cipher.encrypt(
        &lookup_nonce,
        serde_json::to_string(&lookup_table)?.as_bytes(),
    ) {
        Ok(encrypted_lookup_table) => {
            if !lookup_dir_path.exists() {
                fs::create_dir_all(&lookup_dir_path)?;
            }

            if let Err(error) = fs::write(lookup_dir_path.join("nonce"), lookup_nonce) {
                return Err(HimitsuError::StoreNonceError(format!(
                    "Lookup table nonce: {error}"
                )));
            }
            if let Err(error) = fs::write(lookup_dir_path.join("table"), encrypted_lookup_table) {
                return Err(HimitsuError::StoreLookupTableError(error.to_string()));
            }

            Ok(())
        }
        Err(error) => Err(HimitsuError::AEADEncryptionError(format!(
            "Lookup table encryption error: {error}"
        ))),
    }
}
