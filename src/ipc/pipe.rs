use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
    thread,
};

pub fn usage_pipe() {
    let (mut reader,mut writer) = UnixStream::pair().unwrap();

    let handler = thread::spawn(move || {
        let mut buf = [0; 1024];
        let n = reader.read(&mut buf).unwrap();
        println!("Child received: {}", String::from_utf8_lossy(&buf[..n]));
    });

    writer.write_all(b"Hello from parent").unwrap();
    handler.join().unwrap();
}
