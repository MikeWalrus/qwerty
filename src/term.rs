use std::{fs, io::Write, os::unix::prelude::AsRawFd};

use anyhow::Result;
use console::{style, Key, Term};
use termios::{tcsetattr, Termios, ECHO, TCSANOW};

pub fn setup_term() -> Result<Term> {
    let term = Term::stdout();
    set_no_echo()?;
    term.write_line("\n\n")?;
    term.move_cursor_up(2)?;
    Ok(term)
}

fn set_no_echo() -> Result<(), anyhow::Error> {
    // From Term::read_single_key
    let fd = tty_fd()?;
    let mut termios = Termios::from_fd(fd)?;
    termios.c_lflag &= !ECHO;
    tcsetattr(fd, TCSANOW, &termios)?;
    Ok(())
}

fn tty_fd() -> Result<i32> {
    unsafe {
        Ok(if libc::isatty(libc::STDIN_FILENO) == 1 {
            libc::STDIN_FILENO
        } else {
            let tty_f = fs::File::open("/dev/tty")?;
            tty_f.as_raw_fd()
        })
    }
}

struct Answer<'a> {
    pos: usize,
    word: &'a [u8],
}

impl<'a> Answer<'a> {
    fn append_char(&mut self, c: u8) -> std::result::Result<(), ()> {
        let c_correct = self.word[self.pos];
        self.pos += 1;
        if c == c_correct {
            Ok(())
        } else {
            Err(())
        }
    }
}

pub fn prompt_a_word(term: &mut Term, word: &[u8]) -> Result<u32> {
    let len = word.len();
    let mut answer = Answer { pos: 0, word };
    let mut i: u32 = 0;
    'outer: loop {
        term.move_cursor_down(1)?;
        term.write_line(&"~".repeat(len))?;
        term.move_cursor_up(2)?;
        loop {
            let c = term.read_key()?;
            match c {
                Key::Enter => {
                    i = 10;
                    break 'outer;
                }
                Key::Char(c) => {
                    term.write_fmt(format_args!("{}", c as char))?;
                    match answer.append_char(c as u8) {
                        Ok(_) => {
                            change_prev_to_green(term)?;
                            if answer.pos == len {
                                break 'outer;
                            }
                        }
                        Err(_) => {
                            animated_reset(term, &mut answer)?;
                            i += 1;
                            print_misspelt_times(term, i)?;
                            if i > 10 {
                                break 'outer;
                            }
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Ok(i)
}

fn print_misspelt_times(term: &mut Term, i: u32) -> Result<(), anyhow::Error> {
    term.move_cursor_down(2)?;
    term.clear_line()?;
    term.write_fmt(format_args!("Misspelt {} time(s).", i))?;
    term.move_cursor_up(2)?;
    term.clear_line()?;
    Ok(())
}

fn wait_a_bit() {
    std::thread::sleep(std::time::Duration::from_millis(100));
}

fn animated_reset(term: &mut Term, answer: &mut Answer) -> Result<()> {
    term.move_cursor_left(1)?;
    change_to_red(term)?;

    while answer.pos > 0 {
        wait_a_bit();
        answer.pos -= 1;
        term_backspace(term)?;
    }
    Ok(())
}

fn change_to_red(term: &mut Term) -> Result<()> {
    term.move_cursor_down(1)?;
    term.write_fmt(format_args!("{}", style("~").red()))?;
    term.move_cursor_up(1)?;
    Ok(())
}

fn change_prev_to_green(term: &mut Term) -> Result<()> {
    term.move_cursor_left(1)?;
    term.move_cursor_down(1)?;
    term.write_fmt(format_args!("{}", style("~").green()))?;
    term.move_cursor_up(1)?;
    Ok(())
}

fn term_backspace(term: &mut Term) -> Result<()> {
    term.move_cursor_left(1)?;
    term.write_fmt(format_args!(" "))?;
    term.move_cursor_left(1)?;
    Ok(())
}