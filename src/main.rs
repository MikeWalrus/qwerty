#![feature(unix_socket_abstract)]

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

use console::Term;
use qwerty::{
    ipc::Connection,
    term::{prompt_a_word, setup_term},
};

fn main() -> Result<()> {
    let term = setup_term().context("Cannot set up your terminal.")?;
    let mut args = env::args();
    args.next();
    if let Some(file) = args.next() {
        run_on_file(term, file)?;
    } else {
        run_with_connection(term)?;
    }
    Ok(())
}

fn run_on_file(mut term: Term, file: String) -> Result<()> {
    let file = File::open(file)?;
    let file = BufReader::new(file);
    for line in file.lines() {
        let line = line?;
        if !line.is_empty() {
            let word = line.as_bytes();
            prompt_a_word(&mut term, word)?;
        }
    }
    Ok(())
}

fn run_with_connection(mut term: Term) -> Result<()> {
    let mut con = Connection::new()?;
    ctrlc::set_handler(|| eprintln!("Ctrl-C received."))?;
    loop {
        let word = con.receive_a_word()?;
        match word {
            b"/start/" => {}
            b"/exit/" => break,
            _ => {
                match prompt_a_word(&mut term, word) {
                    Ok(i) => {
                        con.send_error_times(i)?;
                    }
                    Err(e) => {
                        if let Some(e) = e.downcast_ref::<std::io::Error>() {
                            if matches!(e.kind(), std::io::ErrorKind::Interrupted) {
                                con.send_quit_message()?;
                                break;
                            }
                        }
                        return Err(e)
                            .context("An error occurred when prompting you to enter the word.");
                    }
                };
            }
        }
    }
    Ok(())
}
