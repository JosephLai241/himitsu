//! `skeletons` - An encrypted CLI tool for managing secret on your machine.

mod authentication;
mod cli;
mod encryption;
mod errors;
mod models;
mod prompts;
mod utils;

use cli::{subcommands, Args};
use errors::SkeletonsError;
use prompts::{authenticate, setup};
use utils::{config, paint};

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
    } else {
        match config::get_encryption_values() {
            Ok(crypt_json) => match crypt_json {
                Some(encryption_values) => {
                    if let Err(error) = authenticate::authenticate_user(&encryption_values) {
                        paint::paint_error(error)
                    } else {
                        if let Some(subcommand) = &args.subcommand {
                            if let Err(error) =
                                subcommands::run_subcommands(&encryption_values, subcommand)
                            {
                                paint::paint_error(error);
                            }
                        }
                    }
                }
                None => {
                    if let Err(error) = setup::run_initial_setup_prompts() {
                        paint::paint_error(error);
                    }

                    match config::get_encryption_values() {
                        Ok(crypt_json) => match crypt_json {
                            Some(encryption_values) => {
                                if let Some(subcommand) = &args.subcommand {
                                    if let Err(error) =
                                        subcommands::run_subcommands(&encryption_values, subcommand)
                                    {
                                        paint::paint_error(error);
                                    }
                                }
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
}
