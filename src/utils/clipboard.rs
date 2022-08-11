//! Contains clipboard utilities for `himitsu`.

use copypasta::{ClipboardContext, ClipboardProvider};

use crate::errors::HimitsuError;

/// Copy the decrypted secret to the system clipboard.
pub fn set_clipboard(secret_value: String) -> Result<(), HimitsuError> {
    match ClipboardContext::new() {
        Ok(mut context) => context.set_contents(secret_value).map_or_else(
            |error| Err(HimitsuError::ClipboardContentsError(error.to_string())),
            Ok,
        ),
        Err(error) => Err(HimitsuError::ClipboardContextError(error.to_string())),
    }
}
