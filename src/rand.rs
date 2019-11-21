#![cfg(target_family = "unix")]

use crate::Error;
use std::fs::File;
use std::io::prelude::Read;

fn read_random(buf: &mut [u8]) -> Result<(), Error> {
    let mut rand_file = match File::open("/dev/urandom") {
        Ok(rand_file) => rand_file,
        Err(err) => return Error::from(err),
    };

    if let Err(err) = rand_file.read(buf) {
        return Error::from(err);
    }

    Ok(())
}

/// Read a random byte from /dev/urandom
pub fn rand_byte() -> Result<u8, Error> {
    let mut buf = vec![0; 1];
    match read_random(&mut buf) {
        Ok(_) => match buf.get(0) {
            Some(b) => Ok(*b),
            None => Error::from("vector out of bounds"),
        },
        Err(err) => Error::from(err),
    }
}

/// Read random bytes from /dev/urandom
pub fn rand_bytes(size: usize) -> Result<Vec<u8>, Error> {
    let mut buf = vec![0; size];
    match read_random(&mut buf) {
        Ok(_) => Ok(buf),
        Err(err) => Error::from(err),
    }
}
