//! `skeletons` - An encrypted CLI tool for managing secret on your machine.

mod authentication;
mod cli;
mod errors;

use cli::Args;

use ansi_term::Color;
use clap::Parser;
use lazy_static::lazy_static;

lazy_static! {
    /// ASCII art for `skeletons`.
    static ref ASCII_ART: &'static [u8; 1424] = include_bytes!("../art.txt");
}

/// Run `skeleton`.
fn main() {
    let args = Args::parse();

    if args.banner {
        println!(
            "{}",
            Color::Fixed(172).paint(String::from_utf8_lossy(&ASCII_ART[..]))
        );
    }
}
