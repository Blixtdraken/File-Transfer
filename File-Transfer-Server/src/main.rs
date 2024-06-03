use std::net::TcpListener;

fn main(){
    println!("Starting...");

    let mut server = TcpListener::bind("0.0.0.0:2345").unwrap();

    loop {

    }

}
