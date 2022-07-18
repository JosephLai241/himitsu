//! `skeletons` - An encrypted CLI tool for managing secret on your machine.

mod authentication;
mod cli;
mod errors;
mod models;
mod prompts;
mod utils;

use cli::Args;
use errors::SkeletonsError;
use prompts::{authenticate, setup};
use utils::{cache, paint};

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

    match cache::get_encryption_values() {
        Ok(crypt_json) => match crypt_json {
            Some(encryption_values) => {
                if let Err(error) = authenticate::authenticate_user(&encryption_values) {
                    paint::paint_error(error)
                } else {
                    // TODO: CONTINUE WITH NORMAL EXECUTION FLOW.
                    unimplemented!()
                }
            }
            None => {
                if let Err(error) = setup::run_initial_setup_prompts() {
                    paint::paint_error(error);
                }

                match cache::get_encryption_values() {
                    Ok(crypt_json) => match crypt_json {
                        Some(encryption_values) => {
                            // TODO: CONTINUE WITH NORMAL EXECUTION FLOW.
                            unimplemented!()
                        }
                        None => paint::paint_error(SkeletonsError::ApplicationError),
                    },
                    Err(error) => paint::paint_error(error),
                }
            }
        },
        Err(error) => paint::paint_error(error),
    }
}
