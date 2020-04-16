//! Configuration utilities

use crate::Fail;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::Read;
use std::str::FromStr;

/// Configuration file parser
#[derive(Clone, Debug)]
pub struct Config<'a> {
    conf: BTreeMap<&'a str, &'a str>,
}

impl<'a> Config<'a> {
    /// Get parsed value from config or default
    pub fn get<T: FromStr>(&self, name: &str, default: T) -> T {
        match self.conf.get(name) {
            Some(value) => value.parse().unwrap_or(default),
            None => default,
        }
    }

    /// Get &str value from config or default
    pub fn value(&self, name: &str, default: &'a str) -> &str {
        match self.conf.get(name) {
            Some(value) => value,
            None => default,
        }
    }

    /// Check whether key exists in config
    pub fn exists(&self, name: &str) -> bool {
        self.conf.contains_key(name)
    }

    /// Get config key/value map
    pub fn conf(&self) -> &BTreeMap<&str, &str> {
        &self.conf
    }

    /// Read config from file
    pub fn read(path: &str, buf: &'a mut String) -> Result<Self, Fail> {
        let mut file = File::open(path).or_else(Fail::from)?;
        match file.read_to_string(buf) {
            Ok(_) => Ok(Self::from(buf)),
            Err(err) => Fail::from(err),
        }
    }

    /// Create new config from raw config string
    pub fn from(raw: &'a str) -> Self {
        // initialize map
        let mut conf = BTreeMap::new();

        // split lines
        raw.split('\n')
            // seperate and trim
            .map(|l| l.splitn(2, '=').map(|c| c.trim()).collect())
            // iterate through seperated lines
            .for_each(|kv: Vec<&str>| {
                // check if contains key and value
                if kv.len() == 2 {
                    // add to map
                    conf.insert(kv[0], kv[1]);
                }
            });

        Self { conf }
    }
}
