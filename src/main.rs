#![allow(non_snake_case)]
#![allow(unused_assignments)]

mod card;
mod deck;
mod token;

// mod symbol;
// mod util;

use crate::{
    card::combineCards,
    deck::Deck,
    token::Token,
    // symbol::Symbol
};

use clap::{value_parser, Arg, Command};
use rand::Rng;
use std::time::Duration;
use std::{fs, path::PathBuf};
use text2art::{BasicFonts, Font, Printer};
use tokio::time::sleep;

fn print(text: String) -> String {
    return Printer::with_font(Font::from_basic(BasicFonts::Bell).unwrap())
        .render_text(text.as_str())
        .unwrap()
        .to_string();
}

#[tokio::main]
async fn main() {
    let LOGO: String = print(format!(
        "{}",
        env!("CARGO_PKG_NAME"),
        // env!("CARGO_PKG_VERSION")
    ));

    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .before_help(LOGO.as_str())
        .help_template("{before-help}{about}\n{usage-heading} {usage}\n\n{all-args}")
        // .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("wager")
                .short('w')
                .default_value("0")
                .value_parser(value_parser!(i32))
                .required(false)
                .help("Set files wager"),
        )
        .arg(
            Arg::new("animation")
                .short('a')
                .action(clap::ArgAction::SetTrue)
                .help("Enable animations"),
        )
        .get_matches();

    let mut WAGER: Token = Token::new(*matches.get_one::<i32>("wager").unwrap());

    if WAGER.value > 99 {
        WAGER.value = 99;
    }

    let mut deck: Deck = Deck::new();
    deck.start();

    while deck.value < 17 {
        sleep(Duration::from_millis(400)).await;
        if deck.value >= 17 {
            break;
        }

        deck.hit();

        if matches.get_flag("animation") {
            clearscreen::clear().expect("Could not clear screen");
            println!("{}", combineCards(deck.cards.clone()).join("\n"));
            println!("{}", print(deck.value.to_string()));
            sleep(Duration::from_millis(800)).await;
        }
    }

    let mut WON: bool = true;

    if deck.value > 21 {
        WON = false
    } else {
        WON = true;
    }

    if matches.get_flag("animation") {
        clearscreen::clear().expect("Could not clear screen");
    }

    println!("{}", combineCards(deck.cards.clone()).join("\n"));

    if WON {
        println!(
            "\x1b[92m{}\x1b[0m",
            print(format!("Game won ({})", deck.value))
        );
    } else {
        println!(
            "\x1b[31m{}\x1b[0m",
            print(format!("Game lost ({})", deck.value))
        );
    }

    if !WON {
        if WAGER.value > 0 {
            let mut files: Vec<PathBuf> = Vec::new();
            for file in std::fs::read_dir(".").unwrap() {
                files.push(file.unwrap().path())
            }

            if WAGER.value > files.len().try_into().unwrap() {
                WAGER.value = files.len().try_into().unwrap();
            }

            let mut deletion: Vec<PathBuf> = Vec::new();
            for _i in 0..WAGER.value {
                deletion.push(getFile(files.clone(), deletion.clone()));
            }

            for file in deletion {
                if file.is_file() {
                    fs::remove_file(file).expect("Failed to delete a file");
                } else if file.is_dir() {
                    fs::remove_dir_all(file).expect("Failed to delete a directory");
                }
            }
        }
    }
}

fn getFile(files: Vec<PathBuf>, deletion: Vec<PathBuf>) -> PathBuf {
    let file = files
        .get(rand::thread_rng().gen_range(0..=files.len() - 1))
        .unwrap();

    if !deletion.contains(file) {
        return file.to_path_buf();
    } else {
        return getFile(files, deletion);
    }
}
