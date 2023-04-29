use std::{io::{self, Read}, fs::File};

pub fn read_file(path: &str) -> io::Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut buf = Vec::new();
    let _ = f.read_to_end(&mut buf)?;

    Ok(buf)
}