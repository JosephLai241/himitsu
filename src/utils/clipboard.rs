//! Contains clipboard utilities for `skeletons`.

use copypasta::{ClipboardContext, ClipboardProvider};

use crate::errors::SkeletonsError;

/// Copy the decrypted secret to the system clipboard.
pub fn set_clipboard(secret_value: &str) -> Result<(), SkeletonsError> {
    match ClipboardContext::new() {
        Ok(mut context) => context.set_contents(secret_value.to_owned()).map_or_else(
            |error| Err(SkeletonsError::ClipboardContentsError(error.to_string())),
            Ok,
        ),
        Err(error) => return Err(SkeletonsError::ClipboardContextError(error.to_string())),
    }
}
