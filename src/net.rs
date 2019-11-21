use crate::Error;
use std::io;
use std::io::prelude::{Read, Write};
use std::net::TcpStream;

/// Stream provides additional self processing functions
pub trait Stream {
    // standard
    fn r(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    fn re(&mut self, buf: &mut [u8]) -> io::Result<()>;
    fn w(&mut self, buf: &[u8]) -> io::Result<usize>;
    fn wa(&mut self, buf: &[u8]) -> io::Result<()>;

    // additional
    fn read_full(&mut self, buf_size: usize) -> Result<Vec<u8>, Error>;
    fn read_byte(&mut self) -> Result<u8, Error>;
    fn read_bool(&mut self) -> Result<bool, Error>;
    fn write_byte(&mut self, byte: u8) -> Result<(), Error>;
    fn write_bool(&mut self, b: bool) -> Result<(), Error>;
}

// Stream implementation for TcpStream
impl Stream for TcpStream {
    // Wrapper for read
    fn r(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read(buf)
    }

    // Wrapper for read_exact
    fn re(&mut self, buf: &mut [u8]) -> io::Result<()> {
        self.read_exact(buf)
    }

    // Wrapper for write
    fn w(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write(buf)
    }

    // Wrapper for write_all
    fn wa(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write_all(buf)
    }

    /// Read until no more data is provided
    fn read_full(&mut self, buf_size: usize) -> Result<Vec<u8>, Error> {
        // initialize vector and loop
        let mut data = Vec::new();
        loop {
            // create buffer and read
            let mut buf = vec![0u8; buf_size];
            let length = match self.read(&mut buf) {
                Ok(length) => length,
                Err(err) => return Error::from(err),
            };

            // check for split and break
            match length {
                len if len < buf_size => {
                    buf.truncate(len);
                    data.append(&mut buf);
                    break;
                }
                _ => data.append(&mut buf),
            }
        }

        // return full data
        Ok(data)
    }

    /// Read byte (u8) from stream
    fn read_byte(&mut self) -> Result<u8, Error> {
        // create buffer and read
        let mut buf = vec![0u8; 1];
        match self.read(&mut buf) {
            Ok(_) => {}
            Err(err) => return Error::from(err),
        }

        // get and return
        match buf.get(0) {
            Some(u) => Ok(*u),
            None => {
                Error::from("unexpected error: can not get first value from vector of length 1")
            }
        }
    }

    /// Read bool from stream
    fn read_bool(&mut self) -> Result<bool, Error> {
        // create buffer and read
        let mut buf = vec![0u8; 1];
        match self.read(&mut buf) {
            Ok(_) => {}
            Err(err) => return Error::from(err),
        }

        // get and compare
        match buf.get(0) {
            Some(u) => {
                if *u == 1u8 {
                    Ok(true)
                } else if *u == 0u8 {
                    Ok(false)
                } else {
                    Error::from("received byte is neither 1 nor 0")
                }
            }
            None => {
                Error::from("unexpected error: can not get first value from vector of length 1")
            }
        }
    }

    /// Write byte (u8) to stream
    fn write_byte(&mut self, u: u8) -> Result<(), Error> {
        // initialize buffer and write
        let buf: &[u8] = &[u];
        match self.write(buf) {
            Ok(_) => Ok(()),
            Err(err) => Error::from(err),
        }
    }

    /// Write bool to stream
    fn write_bool(&mut self, b: bool) -> Result<(), Error> {
        // initialize buffer and write
        let buf: &[u8] = &[if b { 1 } else { 0 }];
        match self.write(buf) {
            Ok(_) => Ok(()),
            Err(err) => Error::from(err),
        }
    }
}
