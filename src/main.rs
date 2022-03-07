#![feature(unix_socket_abstract)]

use anyhow::{Context, Result};

use qwerty::{
    ipc::Connection,
    term::{prompt_a_word, setup_term},
};

fn main() -> Result<()> {
    let mut con = Connection::new()?;
    let mut term = setup_term().context("Cannot set up your terminal.")?;
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
