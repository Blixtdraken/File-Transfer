mod Shared;

use std::io::Read;
use std::net::TcpListener;
use std::thread;


fn main(){
    println!("Starting...");

    let mut server = TcpListener::bind("0.0.0.0:2345").unwrap();

    loop {
        let mut stream = server.accept().unwrap().0;
        let mut thread = thread::spawn(move || {
            loop {
                let mut stream_buffer = [0u8;256];
                Shared::NetworkUtils::fill_buffer_from_stream(&mut stream_buffer, &stream);
                println!("{}", String::from_utf8_lossy(&stream_buffer))
            }
        });
        

    }

}


