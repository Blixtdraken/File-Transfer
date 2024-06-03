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
                let mut streamBuffer = [0u8;256];
                stream.read(&mut streamBuffer);
                println!("{}", String::from_utf8_lossy(&streamBuffer))
            }
        });
        

    }

}


