#![feature(unix_socket_abstract)]

use anyhow::Result;

use qwerty::term::{prompt_a_word, setup_term};

fn main() -> Result<()> {
    let mut term = setup_term()?;
    prompt_a_word(&mut term, b"appleseed")?;
    Ok(())
}
