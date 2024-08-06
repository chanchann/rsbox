use std::{
    error::Error,
    io::{BufReader, BufWriter},
    net::TcpListener,
};

mod handshake;
use handshake::handshake;

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;

    while let Ok((stream, _)) = listener.accept() {
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        handshake(&mut reader, &mut writer)?;
    }

    Ok(())
}
