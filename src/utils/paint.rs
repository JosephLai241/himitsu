//! Contains text painting utilities for `skeleton`.

use ansi_term::Color;

use crate::errors::SkeletonsError;

/// Neatly paints and formats the error raised.
pub fn paint_error(error: SkeletonsError) {
    println!("\n{}\n", Color::Red.bold().paint(error.to_string()));
}
