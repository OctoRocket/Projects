use std::{net::{TcpListener, TcpStream}, io::{Read, Write, stdin}, thread::spawn};
use futures::{executor::block_on};

async fn listen_print(listener: TcpListener) {
    spawn(move || {
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        for i in buffer {
            print!("{}", i as char);
        }
    }});
}

fn main() {
    let mut addr = String::new();
    println!("Enter the address to listen on:");
    stdin().read_line(&mut addr).unwrap();
    let listener = TcpListener::bind(addr.trim()).unwrap();
    block_on(listen_print(listener.try_clone().unwrap()));
    
    spawn(move || {
        let mut stream = TcpStream::connect(addr.trim()).unwrap();
        loop {
            let mut buffer: Vec<u8> = Vec::new();
            let mut message = String::new();
            stdin().read_line(&mut message).unwrap();
            buffer = message.as_bytes().to_vec();
            stream.write(&buffer).unwrap();
        }});
}
