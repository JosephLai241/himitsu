//! Contains the execution flow for the initial setup prompts.

use std::io::Write;

use crate::{authentication, errors::SkeletonsError, models::encryption::Encryption, utils::cache};

use inquire::{self, validator::StringValidator, Password, PasswordDisplayMode};
use rand::{self, distributions::Alphanumeric, Rng};
use spinners::{Spinner, Spinners};

use super::config::get_authentication_config;

/// Run the initial setup's prompt - set a master password to unlock the vault.
pub fn run_initial_setup_prompts() -> Result<Encryption, SkeletonsError> {
    inquire::set_global_render_config(get_authentication_config());

    let password_validator: StringValidator = &|input| {
        if input.chars().count() < 10 {
            Err("The password must have at least 10 characters!".to_string())
        } else {
            Ok(())
        }
    };

    let password = Password::new("Set a master password:")
        .with_display_mode(PasswordDisplayMode::Hidden)
        .with_display_toggle_enabled()
        .with_help_message(
            "Password must have at least 10 characters. Press '<CTRL> + r' to reveal input",
        )
        .with_validator(password_validator)
        .prompt()?;

    let mut loading_bar = Spinner::new(Spinners::Aesthetic, "Generating encryption data...".into());

    let salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let encryption_data = Encryption {
        password_hash: authentication::generate_hash(&password, &salt)?,
        salt,
    };

    let mut crypt_json = cache::get_crypt_json()?;
    crypt_json.write_all(serde_json::to_string(&encryption_data)?.as_bytes())?;

    loading_bar.stop_and_persist("💯", "Successfully set up encryption!".into());

    Ok(encryption_data)
}
