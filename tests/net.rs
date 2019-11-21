extern crate kern;

use kern::net::Stream;
use std::net::{TcpListener, TcpStream};
use std::thread;

#[test]
fn read_full() {
    // create server and client
    let server = TcpListener::bind("127.0.0.1:0").unwrap();
    let mut client = TcpStream::connect(server.local_addr().unwrap()).unwrap();

    // start server listener and read
    let responder = thread::spawn(move || {
        let (mut stream, _) = server.accept().unwrap();
        stream.w("Hallo, das ist ein Test".as_bytes()).unwrap();
    });
    let result = client.read_full(8192).unwrap();

    // check
    assert_eq!(
        "Hallo, das ist ein Test",
        String::from_utf8(result).unwrap()
    );
    responder.join().unwrap();
}

#[test]
fn read_byte() {
    // create server and client
    let server = TcpListener::bind("127.0.0.1:0").unwrap();
    let mut client = TcpStream::connect(server.local_addr().unwrap()).unwrap();

    // start server listener
    let responder = thread::spawn(move || {
        // get stream
        let (mut stream, _) = server.accept().unwrap();

        // write bytes
        stream.w(&vec![96]).unwrap();
        stream.w(&vec![100]).unwrap();
        stream.w(&vec![18]).unwrap();
    });

    // read bytes
    let value_1 = client.read_byte().unwrap();
    let value_2 = client.read_byte().unwrap();
    let value_3 = client.read_byte().unwrap();

    // check
    assert_eq!(96, value_1);
    assert_eq!(100, value_2);
    assert_eq!(18, value_3);
    responder.join().unwrap();
}

#[test]
fn read_bool() {
    // create server and client
    let server = TcpListener::bind("127.0.0.1:0").unwrap();
    let mut client = TcpStream::connect(server.local_addr().unwrap()).unwrap();

    // start server listener
    let responder = thread::spawn(move || {
        // get stream
        let (mut stream, _) = server.accept().unwrap();

        // write bools
        stream.w(&vec![1]).unwrap();
        stream.w(&vec![0]).unwrap();
        stream.w(&vec![1]).unwrap();
    });

    // read bools
    let value_1 = client.read_bool().unwrap();
    let value_2 = client.read_bool().unwrap();
    let value_3 = client.read_bool().unwrap();

    // check
    assert_eq!(true, value_1);
    assert_eq!(false, value_2);
    assert_eq!(true, value_3);
    responder.join().unwrap();
}

#[test]
fn write_byte() {
    // create server and client
    let server = TcpListener::bind("127.0.0.1:0").unwrap();
    let mut client = TcpStream::connect(server.local_addr().unwrap()).unwrap();

    // start server listener
    let responder = thread::spawn(move || {
        // get stream
        let (mut stream, _) = server.accept().unwrap();

        // write bytes
        stream.write_byte(96).unwrap();
        stream.write_byte(100).unwrap();
        stream.write_byte(18).unwrap();
    });

    // read bytes
    let mut values = vec![0u8; 3];
    client.re(&mut values).unwrap();

    // check
    assert_eq!(96, values[0]);
    assert_eq!(100, values[1]);
    assert_eq!(18, values[2]);
    responder.join().unwrap();
}

#[test]
fn write_bool() {
    // create server and client
    let server = TcpListener::bind("127.0.0.1:0").unwrap();
    let mut client = TcpStream::connect(server.local_addr().unwrap()).unwrap();

    // start server listener
    let responder = thread::spawn(move || {
        // get stream
        let (mut stream, _) = server.accept().unwrap();

        // write bools
        stream.write_bool(true).unwrap();
        stream.write_bool(false).unwrap();
        stream.write_bool(true).unwrap();
    });

    // read bools
    let mut values = vec![0u8; 3];
    client.re(&mut values).unwrap();

    // check
    assert_eq!(1, values[0]);
    assert_eq!(0, values[1]);
    assert_eq!(1, values[2]);
    responder.join().unwrap();
}
