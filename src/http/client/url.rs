use std::char;

use crate::Fail;

pub struct Url<'a> {
    pub secure: bool,
    pub addr: String,
    pub path: &'a str,
    pub server_name: &'a str,
}

impl<'a> TryFrom<&'a str> for Url<'a> {
    type Error = Box<Fail>;

    fn try_from(url: &'a str) -> std::result::Result<Self, Box<Fail>> {
        let (url, secure) = if let Some(secure_url) = url.strip_prefix("https://") {
            (secure_url, true)
        } else {
            (
                url.strip_prefix("http://")
                    .ok_or_else(|| Fail::new("invalid url: protocol missing"))?,
                false,
            )
        };

        let (raw_addr, path) = url.split_once('/').unwrap_or((url, ""));
        let mut addr = None;
        let (server_name, _) = raw_addr.split_once(':').unwrap_or_else(|| {
            addr = Some(format!("{}:{}", raw_addr, if secure { 443 } else { 80 }));
            (raw_addr, "")
        });

        Ok(Self {
            secure,
            addr: addr.unwrap_or(raw_addr.into()),
            path,
            server_name,
        })
    }
}

pub fn url_encode(url: impl AsRef<str>, skip_slash: bool) -> String {
    let url = url.as_ref();
    let mut encoded = String::with_capacity(url.len());
    let mut bytes = [0u8; 4];

    url.chars().for_each(|c| {
        if matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '.' | '_' | '~')
            || (skip_slash && c == '/')
        {
            encoded.push(c);
        } else {
            c.encode_utf8(&mut bytes);
            for byte in bytes.iter().take(c.len_utf8()) {
                encoded.push('%');
                encoded.push(to_hex(byte >> 4));
                encoded.push(to_hex(byte & 15));
            }
        }
    });
    encoded
}

fn to_hex(byte: u8) -> char {
    if byte < 10 {
        (b'0' + byte) as char
    } else {
        (b'A' - 10 + byte) as char
    }
}

#[test]
fn test_url_encode() {
    assert_eq!(
        "%20%21%22%23%24%25%26%27%28%29%2A%2B%2C-.%2F%3A%3B%3C%3D%3E%3F%40%5B%5C%5D%7B%7C%7D",
        url_encode(" !\"#$%&'()*+,-./:;<=>?@[\\]{|}", false)
    );
    assert_eq!(
        "%20%21%22%23%24%25%26%27%28%29%2A%2B%2C-./%3A%3B%3C%3D%3E%3F%40%5B%5C%5D%7B%7C%7D",
        url_encode(" !\"#$%&'()*+,-./:;<=>?@[\\]{|}", true)
    );
}
