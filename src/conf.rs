use crate::Error;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::prelude::Read;

/// Configuration file parser
pub struct Config {
    conf: BTreeMap<String, String>,
}

impl Config {
    /// Get value from config
    pub fn get(&self, name: &str) -> Option<&str> {
        match self.conf.get(name) {
            Some(value) => Some(&value),
            None => None,
        }
    }

    /// Check whether key exists in config
    pub fn exists(&self, name: &str) -> bool {
        self.conf.contains_key(name)
    }

    /// Check whether values match in config
    pub fn equals(&self, name: &str, value: &str) -> bool {
        match self.conf.get(name) {
            Some(orig) => orig == value,
            None => false,
        }
    }

    /// Fill config with full config
    pub fn fill(mut self, raw: &str) -> Self {
        let conf = config_map(raw);
        for (key, value) in conf.iter() {
            if !self.conf.contains_key(key) {
                self.conf.insert(key.to_string(), value.to_string());
            }
        }
        self
    }

    /// Read config from file
    pub fn read(path: &str) -> Result<Self, Error> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => return Error::from(err),
        };
        let mut buf = String::new();
        match file.read_to_string(&mut buf) {
            Ok(_) => Ok(Self::from(&buf)),
            Err(err) => Error::from(err),
        }
    }

    /// Create new config from raw config string
    pub fn from(raw: &str) -> Self {
        let conf = config_map(raw);
        Self { conf }
    }
}

// create config map from string
fn config_map(raw: &str) -> BTreeMap<String, String> {
    // empty map and split
    let mut conf: BTreeMap<String, String> = BTreeMap::new();
    let lines = raw.split('\n');

    // loop through lines
    for line in lines {
        // check if not empty
        if line.is_empty() {
            continue;
        }

        // split key and value
        let kv: Vec<&str> = line.splitn(2, '=').collect();
        let key = match kv.get(0) {
            Some(key) => key.trim().to_string(),
            None => String::new(),
        };
        let value = match kv.get(1) {
            Some(value) => value.trim().to_string(),
            None => String::new(),
        };

        // add to conf map
        conf.insert(key, value);
    }
    // return
    conf
}
