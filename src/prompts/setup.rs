//! Contains the execution flow for the initial setup prompts.

use std::io::Write;

use ansi_term::Color;
use inquire::{self, validator::StringValidator, Password, PasswordDisplayMode};
use rand::{self, rngs::OsRng, RngCore};
use serde_json;
use spinners::{Spinner, Spinners};

use crate::{
    authentication, errors::HimitsuError, lookup::init, models::encryption::Encryption,
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

    println!(
        "                            {}\n",
        Color::Fixed(172)
            .blink()
            .bold()
            .reverse()
            .underline()
            .paint("welcome")
    );

    match Password::new("Set up a password for your vault:")
        .with_display_mode(PasswordDisplayMode::Hidden)
        .with_display_toggle_enabled()
        .with_help_message(
            "Password must have at least 10 characters. Press \"<CTRL> + r\" to reveal input",
        )
        .with_render_config(get_inquire_config(ConfigType::Standard, true))
        .with_validator(password_validator)
        .prompt_skippable()?
    {
        Some(password) => {
            let mut loading_bar =
                Spinner::new(Spinners::Aesthetic, "Generating encryption data...".into());

            let encryption_data = generate_salt_and_password_hash(&password)?;
            init::create_lookup(&password)?;

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
        None => Err(HimitsuError::UserCancelled),
    }
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
