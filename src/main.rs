extern crate ansi_term;
extern crate rusqlite;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};

use ansi_term::Colour;
use text_io::read;

#[derive(Serialize, Deserialize)]
struct Card {
    question: String,
    answer: String,
    score: f32,
}

fn enter_new_card(conn: &mut Connection) -> Result<()> {
    // Connect to our DB
    let tx = conn.transaction()?;

    // Ask user for input
    loop {
        println!("{}", Colour::Blue.paint("Enter a question: "));
        let input_question: String = read!("{}\n");
        println!("{}", Colour::Blue.paint("Enter the answer: "));
        let input_answer: String = read!("{}\n");

        // Create our card struct
        let edu_card: Card = Card {
            question: input_question,
            answer: input_answer,
            score: 0.0,
        };

        // add it to our database
        tx.execute()

        // Ask user if they want to repeat
        println!(
            "{}",
            Colour::Red.paint("Do you want to insert another card?\n [y] - yes, [n] - no")
        );
        let user_input = read!("{}\n");
        if user_input != "y" {
            break;
        }
    }

    tx.commit();
}

fn main() -> Result<()> {
    println!(
        "{}",
        Colour::Green
            .bold()
            .paint("=================  Welcome to Edulingo! ===================")
    );

    let conn = Connection::open("edulingo.db")?;

    conn.execute(
        "create table if not exists edu_cards (
             id integer primary key,
             question text not null unique,
             answer text not null,
             score real not null
         )",
        NO_PARAMS,
    )?;

    loop {
        println!(
            "{}",
            Colour::Yellow.bold().paint("Please select an option!")
        );
        println!("{}", Colour::Green.paint("[!c] - Create a card"));
        println!("{}", Colour::Red.paint("[!s] - Study"));
        println!("{}", Colour::Green.paint("[!e] - Edit a card"));
        println!("{}", Colour::Red.paint("[!q] - Quit"));
        let input_control: String = read!("{}\n");

        if input_control == "!c" {
            enter_new_card();
        } else if input_control == "!q" {
            break;
        } else if input_control == "!s" {
            break;
        }
    }

    Ok(())
}
