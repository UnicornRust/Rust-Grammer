use std::{fs, io::{Read, Write}, os::unix::net::{SocketAddr, UnixListener, UnixStream}, thread};



const SOCK_PATH: &str = "/tmp/example.sock";

pub fn usage_unix() {

    // Clean up from previous run
    let _ = fs::remove_file(SOCK_PATH);

    let listener = UnixListener::bind(SOCK_PATH).unwrap();

    let handle = thread::spawn(move || {
        let mut stream = UnixStream::connect(SOCK_PATH).unwrap();
        stream.write_all(b"Hello from client!").unwrap();
    });

    let (mut stream, _) = listener.accept().unwrap();
    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).unwrap();
    println!("Server received: {}", String::from_utf8_lossy(&buf[..n]));
    fs::remove_file(SOCK_PATH).unwrap();
}
