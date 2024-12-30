use std::fs;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for ele in listener.incoming() {
        let stream: TcpStream = ele.unwrap();
        handle_request(stream);
    }
}

///
/// chrome失败的原因：
/// https://users.rust-lang.org/t/cant-write-to-tcpstream-without-reading-from-it/16713/2
/// 
/// answer:
/// You have to read the entire request or else the client blocks and then fails as soon as you close the socket. 
/// It never actually tries to read your response before it has sent the request completely.
/// The code is problematic because it relies on OS buffering on the client or server side.
/// Without buffering, a peer can only send if the other peer is reading and vice versa. With buffering, the same problem exists but only if the buffers are full.
/// 
/// buffer大小是512时，postman可以，chrome不行。1024时，都可以
///
fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let html = fs::read_to_string("hello.html").unwrap();
    let contents = format!("HTTP/1.1 200 OK\r\n\r\n{}", html);

    stream.write(contents.as_bytes()).unwrap();
    stream.flush().unwrap();
}
