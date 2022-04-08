use std::{net::{TcpListener}, io::{Read, Write, stdin}};
use futures::{executor::block_on};

async fn listen_print(listener: TcpListener) {
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        for i in buffer {
            print!("{}", i as char);
        }
    }
}

fn main() {
    let mut addr = String::new();
    println!("Enter the address to listen on:");
    stdin().read_line(&mut addr).unwrap();
    let listener = TcpListener::bind(addr.trim()).unwrap();
    block_on(listen_print(listener.try_clone().unwrap()));
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut message = String::new();
        stdin().read_line(&mut message).unwrap();
        let mut buffer = [0; 1024];
        for i in message.as_bytes() {
            buffer[*i as usize] = *i;
        }
        stream.write(&mut buffer).unwrap();
    }
}
