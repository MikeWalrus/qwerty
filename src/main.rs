#![feature(unix_socket_abstract)]

use anyhow::{Context, Result};

use qwerty::{
    ipc::Connection,
    term::{prompt_a_word, setup_term},
};

fn main() -> Result<()> {
    let mut con = Connection::new()?;
    let mut term = setup_term().context("Cannot setup your terminal.")?;
    let word = con.receive_a_word()?;
    let error_times =
        prompt_a_word(&mut term, word).context("An error occurred when prompting the word.")?;
    con.send_error_times(error_times)?;
    Ok(())
}
