//! Database

use crate::{Fail, Result};
use std::collections::HashMap;
use std::fs::{remove_file, rename, File, OpenOptions};
use std::io::prelude::*;

/// Raw data storage file
#[derive(Debug)]
pub struct StorageFile {
    file: File,
    raw: String,
    cache: HashMap<String, String>,
}

impl StorageFile {
    /// Open file or create new
    pub fn new(file_name: impl AsRef<str>) -> Result<Self> {
        // open file and parse
        let mut file = open_file(file_name)?;
        let raw = read_file(&mut file)?;
        let raw = String::from_utf8(raw)?;
        let cache = parse(&raw);

        // return
        Ok(Self { file, raw, cache })
    }

    /// Get raw
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// Write directly to file and parse
    pub fn raw_write(&mut self, raw: String) -> Result<()> {
        // parse and write to file
        self.raw = raw;
        self.cache = parse(&self.raw);
        write_file(&mut self.file, &self.raw)
    }

    /// Get map from cache
    pub fn cache(&self) -> &HashMap<String, String> {
        &self.cache
    }

    /// Get map from cache mutably
    pub fn cache_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.cache
    }

    /// Serialize map to string and write to file
    pub fn write(&mut self) -> Result<()> {
        // serialize and write
        self.raw = serialize(self.cache());
        write_file(&mut self.file, &self.raw)
    }
}

/// Open file or create new
pub fn open_file(file_name: impl AsRef<str>) -> Result<File> {
    // open and return file
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name.as_ref())
        .or_else(Fail::from)
}

/// Delete file if exists
pub fn delete_file(file_name: impl AsRef<str>) -> Result<()> {
    // delete file
    remove_file(file_name.as_ref()).or_else(Fail::from)
}

/// Move file
pub fn move_file(file_name: impl AsRef<str>, new_file_name: impl AsRef<str>) -> Result<()> {
    // delete file
    rename(file_name.as_ref(), new_file_name.as_ref()).or_else(Fail::from)
}

/// Read data from file
pub fn read_file(file: &mut File) -> Result<Vec<u8>> {
    // start from beginning
    file.rewind()?;

    // create buffer
    let mut buf = Vec::with_capacity(match file.metadata() {
        Ok(metadata) => metadata.len() as usize,
        Err(_) => 8192,
    });

    // read and return
    file.read_to_end(&mut buf)?;
    Ok(buf)
}

/// Write data to file
pub fn write_file(file: &mut File, data: impl AsRef<[u8]>) -> Result<()> {
    // truncate file
    file.set_len(0)?;

    // start from first byte
    file.rewind()?;

    // write data
    file.write_all(data.as_ref()).or_else(Fail::from)
}

/// Parse storage file buf to map
pub fn parse(buf: &str) -> HashMap<String, String> {
    // initialize map and split lines
    let mut conf = HashMap::new();
    buf.split('\n')
        // seperate and trim
        .map(|l| l.splitn(2, '=').map(|c| c.trim()).collect())
        // iterate through seperated lines
        .for_each(|kv: Vec<&str>| {
            // check if contains key and value
            if kv.len() == 2 {
                conf.insert(kv[0].to_lowercase(), kv[1].to_string());
            }
        });

    // return
    conf
}

/// Serialize map to string
pub fn serialize(data: &HashMap<String, String>) -> String {
    // create buffer
    let mut buf = String::with_capacity(data.len() * 10);

    // add entries
    for (k, v) in data {
        buf.push_str(k);
        buf.push('=');
        buf.push_str(v);
        buf.push('\n');
    }

    // return
    buf
}
