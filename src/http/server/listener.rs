//! TCP listener

use crate::http::server::{accept_connections, Handler, HttpSettings};
use crate::Fail;

use rustls::server::ServerConfig;
use rustls::{Certificate, PrivateKey};
use rustls_pemfile::Item::{PKCS8Key, RSAKey};
use rustls_pemfile::{certs, read_one};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

/// Listen on TCP
pub fn listen<T: Send + Sync + 'static>(
    addr: &str,
    threads: u8,
    http_settings: HttpSettings,
    tls_config: ServerConfig,
    handler: Handler<T>,
    shared: Arc<RwLock<T>>,
) -> Result<Vec<JoinHandle<()>>, Fail> {
    // listen
    let listener = TcpListener::bind(addr).or_else(Fail::from)?;
    let listener = Arc::new(RwLock::new(listener));

    // config
    let http_settings = Arc::new(http_settings);
    let tls_config = Arc::new(tls_config);

    // start threads
    let mut handler_threads = Vec::new();
    (0..threads).for_each(|_| {
        // clones
        let listener = listener.clone();
        let http_settings = http_settings.clone();
        let tls_config = tls_config.clone();
        let shared = shared.clone();

        // spawn thread
        handler_threads.push(thread::spawn(move || {
            accept_connections(listener, http_settings, tls_config, handler, shared)
        }));
    });

    // return threads
    Ok(handler_threads)
}

/// Generate config with TLS certificate and private key
pub fn certificate_config(raw_cert: &[u8], raw_key: &[u8]) -> Result<ServerConfig, Fail> {
    // create config
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    // open certificate
    let mut cert_buf = BufReader::new(raw_cert);
    let cert = certs(&mut cert_buf)
        .or_else(|_| Fail::from("broken certificate"))?
        .iter()
        .map(|v| Certificate(v.clone()))
        .collect();

    // open private key
    let mut key_buf = BufReader::new(raw_key);
    let key = match read_one(&mut key_buf).or_else(|_| Fail::from("broken private key"))? {
        Some(RSAKey(key)) => PrivateKey(key),
        Some(PKCS8Key(key)) => PrivateKey(key),
        _ => return Fail::from("broken private key"),
    };

    // return config with certificate
    config.with_single_cert(cert, key).or_else(Fail::from)
}

/// Generate config with TLS certificate and private key from file
pub fn load_certificate(cert_path: &str, key_path: &str) -> Result<ServerConfig, Fail> {
    // open files
    let mut cert_file = File::open(cert_path).or_else(Fail::from)?;
    let mut key_file = File::open(key_path).or_else(Fail::from)?;

    // create buffers
    let mut cert_buf = Vec::new();
    let mut key_buf = Vec::new();

    // read files
    cert_file.read_to_end(&mut cert_buf).or_else(Fail::from)?;
    key_file.read_to_end(&mut key_buf).or_else(Fail::from)?;

    // generate config and return
    certificate_config(&cert_buf, &key_buf)
}
