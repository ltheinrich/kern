//! TLS utils

use crate::{Fail, Result};

use rustls::server::ServerConfig;
use rustls_pemfile::Item::{Pkcs1Key, Pkcs8Key, Sec1Key};
use rustls_pemfile::{certs, read_one};
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::sync::{Arc, OnceLock};

pub type TlsConfig = Arc<ServerConfig>;
pub type TlsConfigProvider = fn() -> TlsConfig;

static TLS_CONFIG_LC: OnceLock<TlsConfig> = OnceLock::new();
static TLS_CONFIG_CC: OnceLock<TlsConfig> = OnceLock::new();

/// Initializes and returns a single TlsConfigProvider (using load_certificate)<br>
/// The first call sets the TlsConfig, any following call will only return the same provider as the first call!
pub fn load_certificate_provider(
    cert_path: impl AsRef<str>,
    key_path: impl AsRef<str>,
) -> Result<TlsConfigProvider> {
    let tls_config = load_certificate(cert_path, key_path)?;
    TLS_CONFIG_LC.get_or_init(|| tls_config);
    Ok(provide_tls_config_lc)
}

/// Initializes and returns a single TlsConfigProvider (using certificate_config)<br>
/// The first call sets the TlsConfig, any following call will only return the same provider as the first call!
pub fn certificate_config_provider(
    raw_cert: impl AsRef<[u8]>,
    raw_key: impl AsRef<[u8]>,
) -> Result<TlsConfigProvider> {
    let tls_config = certificate_config(raw_cert, raw_key)?;
    TLS_CONFIG_CC.get_or_init(|| tls_config);
    Ok(provide_tls_config_cc)
}

fn provide_tls_config_lc() -> TlsConfig {
    TLS_CONFIG_LC.get().unwrap().clone()
}

fn provide_tls_config_cc() -> TlsConfig {
    TLS_CONFIG_CC.get().unwrap().clone()
}

/// Generate config with TLS certificate and private key
pub fn certificate_config(
    raw_cert: impl AsRef<[u8]>,
    raw_key: impl AsRef<[u8]>,
) -> Result<TlsConfig> {
    // create config
    let config = ServerConfig::builder().with_no_client_auth();

    // open certificate
    let mut cert_buf = BufReader::new(raw_cert.as_ref());
    let cert = certs(&mut cert_buf)
        .map(|v| v.or_else(|_| Fail::from("broken certificate")))
        .collect::<Result<Vec<CertificateDer>>>()?;

    // open private key
    let mut key_buf = BufReader::new(raw_key.as_ref());
    let key: PrivateKeyDer =
        match read_one(&mut key_buf).or_else(|_| Fail::from("broken private key"))? {
            Some(Pkcs1Key(key)) => key.into(),
            Some(Pkcs8Key(key)) => key.into(),
            Some(Sec1Key(key)) => key.into(),
            _ => return Fail::from("broken private key"),
        };

    // return config with certificate
    config
        .with_single_cert(cert, key)
        .map(Arc::new)
        .or_else(Fail::from)
}

/// Generate config with TLS certificate and private key from file
pub fn load_certificate(
    cert_path: impl AsRef<str>,
    key_path: impl AsRef<str>,
) -> Result<TlsConfig> {
    // open files
    let mut cert_file = File::open(cert_path.as_ref())?;
    let mut key_file = File::open(key_path.as_ref())?;

    // create buffers
    let mut cert_buf = Vec::new();
    let mut key_buf = Vec::new();

    // read files
    cert_file.read_to_end(&mut cert_buf)?;
    key_file.read_to_end(&mut key_buf)?;

    // generate config and return
    certificate_config(&cert_buf, &key_buf)
}
