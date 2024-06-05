use std::io::{ErrorKind, Read};
use std::net::TcpStream;
use std::ptr::read;
use std::thread::scope;

pub fn fill_buffer_from_stream(buffer: &mut [u8], mut stream: &TcpStream){
    let mut stream_buffer = &&&&&buffer;
    loop{

        let data_size:usize = match stream.peek(&mut stream_buffer){
            Ok(n) if n == 0=>0,
            Ok(n)=>n,
            Err(e) if e.kind() == ErrorKind::WouldBlock=>continue,
            Err(e)=> panic!("Error {:?}", e),
        };

        if data_size == stream_buffer.len() {
            break;
        }
        //println!("Peeked {} bytes",data_size)
    }

    stream.read(&mut stream_buffer);





}