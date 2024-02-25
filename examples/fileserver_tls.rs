extern crate kern;

use kern::http::name;
use kern::http::server::{load_certificate, respond, HttpSettings, ResponseData};
use kern::http::server::{HttpRequest, HttpServerBuilder};
use kern::meta::version;
use kern::{Error, Fail, Result};
use std::fs::File;
use std::io::prelude::Read;
use std::sync::{Arc, RwLock};

fn main() {
    let tls_config = load_certificate("examples/cert.pem", "examples/key.pem").unwrap();
    let settings = HttpSettings::new();
    let server = HttpServerBuilder::new()
        .addr("[::]:8443")
        .threads(4)
        .settings(settings)
        .tls(tls_config)
        .handler(handler)
        .error_handler(error_handler)
        .build(Arc::new(RwLock::new(0u32)))
        .unwrap();
    server.block().unwrap();
}

fn handler(req: HttpRequest, shared: Arc<RwLock<u32>>) -> Result<Vec<u8>> {
    let mut num = shared.write().unwrap();
    *num += 1;
    dbg!(*num);
    println!("New request from IP: {}", req.ip());
    let filename = req
        .get()
        .get("file")
        .ok_or_else(|| Fail::new("filename missing, try adding ?file=... to the url"))?;
    let mut file = File::open(filename)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(respond(buf, "text/html", None))
}

fn error_handler(err: Error, _: Arc<RwLock<u32>>) -> Vec<u8> {
    let msg = format!(
        "<!DOCTYPE html><html><head><title>{0}</title></head><body><h3>Fileserver error</h3><p>{0}</p><hr><address>{1} v{2}</address></body></html>",
        err,
        name(),
        version()
    );
    respond(
        msg.into_bytes(),
        "text/html",
        ResponseData::bad_request().build(),
    )
}
