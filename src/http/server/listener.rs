//! TCP listener

use crate::http::server::{accept_connections, Handler, HttpSettings};
use crate::{Fail, Result};

use rustls::server::ServerConfig;
use rustls_pemfile::Item::{Pkcs1Key, Pkcs8Key, Sec1Key};
use rustls_pemfile::{certs, read_one};
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
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
) -> Result<Vec<JoinHandle<()>>> {
    // listen
    let listener = TcpListener::bind(addr)?;
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
pub fn certificate_config(raw_cert: &[u8], raw_key: &[u8]) -> Result<ServerConfig> {
    // create config
    let config = ServerConfig::builder().with_no_client_auth();

    // open certificate
    let mut cert_buf = BufReader::new(raw_cert);
    let cert = certs(&mut cert_buf)
        .map(|v| v.or_else(|_| Fail::from("broken certificate")))
        .collect::<Result<Vec<CertificateDer>>>()?;

    // open private key
    let mut key_buf = BufReader::new(raw_key);
    let key: PrivateKeyDer =
        match read_one(&mut key_buf).or_else(|_| Fail::from("broken private key"))? {
            Some(Pkcs1Key(key)) => key.into(),
            Some(Pkcs8Key(key)) => key.into(),
            Some(Sec1Key(key)) => key.into(),
            _ => return Fail::from("broken private key"),
        };

    // return config with certificate
    config.with_single_cert(cert, key).or_else(Fail::from)
}

/// Generate config with TLS certificate and private key from file
pub fn load_certificate(cert_path: &str, key_path: &str) -> Result<ServerConfig> {
    // open files
    let mut cert_file = File::open(cert_path)?;
    let mut key_file = File::open(key_path)?;

    // create buffers
    let mut cert_buf = Vec::new();
    let mut key_buf = Vec::new();

    // read files
    cert_file.read_to_end(&mut cert_buf)?;
    key_file.read_to_end(&mut key_buf)?;

    // generate config and return
    certificate_config(&cert_buf, &key_buf)
}
