extern crate ansi_term;
extern crate rusqlite;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};

use ansi_term::Colour;
use text_io::read;

#[derive(Serialize, Deserialize, Debug)]
struct Card {
    question: String,
    answer: String,
}

fn enter_new_card(conn: &mut Connection) -> Result<()> {
    // Connect to our DB
    let tx = conn.transaction()?;

    // Ask user for input
    println!("{}", Colour::Blue.paint("Enter a question: "));
    let input_question: String = read!("{}\n");
    println!("{}", Colour::Blue.paint("Enter the answer: "));
    let input_answer: String = read!("{}\n");

    // add it to our database
    tx.execute(
        "INSERT INTO edu_cards (question, answer, score) values(?1, ?2, 0.0);",
        &[input_question, input_answer],
    )?;

    tx.commit()
}

fn display_cards(conn: &mut Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM edu_cards;")?;

    let cards = stmt.query_map(NO_PARAMS, |row| {
        Ok(Card {
            question: row.get(1)?,
            answer: row.get(2)?,
        })
    })?;

    for card in cards {
        println!("{:?}", card);
    }

    Ok(())
}

fn main() -> Result<()> {
    println!(
        "{}",
        Colour::Green
            .bold()
            .paint("=================  Welcome to Edulingo! ===================")
    );

    let mut conn = Connection::open("edulingo.db")?;

    conn.execute(
        "create table if not exists edu_cards (
             id integer primary key,
             question text not null unique,
             answer text not null,
             score real not null
         );",
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
            enter_new_card(&mut conn);
        } else if input_control == "!s" {
            display_cards(&mut conn);
        } else if input_control == "!q" {
            break;
        }
    }

    Ok(())
}
