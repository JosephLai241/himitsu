//! Contains the execution flow for authentication.

use crate::{authentication, errors::HimitsuError, models::encryption::Encryption};

use ansi_term::Color;
use inquire::{self, Password, PasswordDisplayMode};
use spinners::{Spinner, Spinners};

use super::config::{self, ConfigType};

/// Run the authentication prompt.
pub fn authenticate_user(encryption_values: &Encryption) -> Result<String, HimitsuError> {
    let mut try_count = 0;

    while try_count < 3 {
        let password = Password::new("Enter the password:")
            .with_display_mode(PasswordDisplayMode::Hidden)
            .with_display_toggle_enabled()
            .with_help_message("Press \"<CTRL> + r\" to reveal input")
            .with_render_config(config::get_inquire_config(ConfigType::Standard, true))
            .prompt_skippable()?;

        match password {
            Some(input) => {
                let mut login_spinner = Spinner::new(Spinners::Aesthetic, "Logging in...".into());

                if input.is_empty() {
                    login_spinner.stop_and_persist(
                        "❗️",
                        format!(
                            "{}",
                            Color::Fixed(172).bold().paint("Please enter a password.")
                        ),
                    );
                } else if !authentication::check_authorization(encryption_values, &input)? {
                    if try_count < 2 {
                        login_spinner.stop_and_persist(
                                "🤔",
                                format!(
                                    "{}",
                                    Color::Fixed(172).bold().paint(
                                        "Looks like you're not who we think you are. Perhaps try again?"
                                    )
                                ),
                            );
                    } else {
                        login_spinner.stop_and_persist(
                            "🥴",
                            format!("{}", Color::Red.bold().paint("FAILED TO AUTHENTICATE.")),
                        );
                    }

                    try_count += 1;
                } else {
                    login_spinner.stop_and_persist(
                        "💯",
                        format!("{}", Color::Green.bold().paint("Success.")),
                    );

                    return Ok(input);
                }
            }
            None => return Err(HimitsuError::FailedToLogin),
        }
    }

    Err(HimitsuError::FailedToLogin)
}
