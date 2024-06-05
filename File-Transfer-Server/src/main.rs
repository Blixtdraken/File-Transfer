mod Shared;

use std::io::Read;
use std::net::TcpListener;
use std::thread;


fn main(){
    println!("Starting...");

    let mut server = TcpListener::bind("0.0.0.0:2345").unwrap();

    loop {
        let mut stream = server.accept().unwrap().0;
        stream.set_nonblocking(true);
        thread::spawn(move || {
            println!("Got connection from {} to  {}", stream.peer_addr().unwrap().to_string(), stream.local_addr().unwrap().to_string());
            'main: loop {
                let mut stream_buffer = [0u8;256];
                Shared::NetworkUtils::fill_buffer_from_stream(&mut stream_buffer, &stream);
                println!("{}", String::from_utf8_lossy(&stream_buffer));
                println!("{}", Shared::NetworkUtils::unbuffer_string(&stream_buffer) == "send")
            }
        });
        

    }

}


