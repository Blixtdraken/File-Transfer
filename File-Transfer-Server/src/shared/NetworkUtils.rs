use std::io::{ErrorKind, Read};
use std::net::TcpStream;
use std::ptr::read;
use std::thread;
use std::thread::scope;

pub fn fill_buffer_from_stream(buffer: &mut [u8], mut stream: &TcpStream) -> bool{
    let mut stream_buffer = buffer;
    let mut should_break = false;
    loop{

        let data_size:usize = match stream.peek(&mut stream_buffer){
            Ok(n) if n == 0=>0,
            Ok(n)=>n,
            Err(e) if e.kind() == ErrorKind::WouldBlock=>continue,
            Err(e)=> {
                should_break = true;
                break
            },
        };

        if data_size == stream_buffer.len() {
            break;
        }
        //println!("Peeked {} bytes",data_size)
    }
    if should_break {
        return true;
    }

    stream.read(&mut stream_buffer);

    return false;

}

pub fn place_bytes_in_buffer(bytes:&[u8], buffer: &mut [u8]){
    for i in 0..bytes.len(){
        buffer[i] = bytes[i];
    }
}

pub fn unbuffer_string(buffer:&[u8]) -> String {
    let string = String::from_utf8_lossy(buffer).to_string();
    let string=    string.trim_matches(char::from(0));

    return string.to_string();

}

