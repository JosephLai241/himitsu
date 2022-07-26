//! Contains text painting utilities for `himitsu`.

use ansi_term::Color;

use crate::errors::HimitsuError;

/// Neatly paints and formats the error raised.
pub fn paint_error(error: HimitsuError) {
    println!("\n{}\n", Color::Red.bold().paint(error.to_string()));
}
