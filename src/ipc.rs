use anyhow::{anyhow, Result};
use std::{fs, os::unix::net::UnixDatagram, path::Path};

pub struct Connection {
    socket: UnixDatagram,
    buf: [u8; 128],
}

impl Connection {
    pub fn new() -> Result<Self> {
        let in_socket_path = Path::new("/tmp/qwerty.socket");
        let out_socket_path = Path::new("/tmp/qwerty_anki.socket");
        if in_socket_path.exists() {
            fs::remove_file(in_socket_path)?
        }
        println!("Waiting for connection...");
        let socket = UnixDatagram::bind(in_socket_path)?;
        let mut con = Self {
            socket,
            buf: [0; 128],
        };
        let word = con.receive_a_word()?;
        match word {
            b"/start/" => {
                con.socket.connect(out_socket_path)?;
                println!("Connected.");
            }
            _ => return Err(anyhow!("Error while establishing connection.")),
        }
        Ok(con)
    }

    pub fn receive_a_word(&mut self) -> Result<&[u8]> {
        let len = self.socket.recv(&mut self.buf)?;
        Ok(&self.buf[0..len])
    }

    pub fn send_error_times(&self, error_times: u32) -> Result<()> {
        self.socket.send(error_times.to_string().as_bytes())?;
        Ok(())
    }

    pub fn send_quit_message(&self) -> Result<()> {
        eprintln!("Telling qwerty_anki we will quit...");
        self.socket.send("quit".as_bytes())?;
        Ok(())
    }
}
