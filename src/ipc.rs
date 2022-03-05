use anyhow::{anyhow, Result};
use std::{fs, os::unix::net::UnixDatagram, path::Path};

pub struct Connection {
    socket: UnixDatagram,
    out_socket: UnixDatagram,
    buf: [u8; 128],
}

impl Connection {
    pub fn new() -> Result<Self> {
        let in_socket_path = Path::new("/tmp/qwerty.socket");
        let out_socket_path = Path::new("/tmp/qwerty_anki.socket");
        let out_socket = UnixDatagram::unbound()?;
        if in_socket_path.exists() {
            fs::remove_file(in_socket_path)?
        }
        println!("Waiting for connection...");
        let socket = UnixDatagram::bind(in_socket_path)?;
        let mut con = Self {
            socket,
            out_socket,
            buf: [0; 128],
        };
        let word = con.receive_a_word()?;
        match word {
            "start" => {
                con.out_socket.connect(out_socket_path)?;
                println!("Connected.");
            }
            _ => return Err(anyhow!("Error ")),
        }
        Ok(con)
    }

    pub fn receive_a_word(&mut self) -> Result<&str> {
        let len = self.socket.recv(&mut self.buf)?;
        Ok(std::str::from_utf8(&self.buf[..len])?)
    }

    pub fn send_error_times(&mut self, error_times: i32) -> Result<()> {
        self.out_socket.send(error_times.to_string().as_bytes())?;
        Ok(())
    }
}
