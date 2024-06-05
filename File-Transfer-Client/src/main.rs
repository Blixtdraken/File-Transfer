mod FileSystem;
mod CommandHandler;
mod Shared;


use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::{fs, io, thread};
use std::io::Write;
use std::ops::Deref;
use std::path::Path;
use std::time::Duration;


fn main() {
    println!("Booting up...");
    print!("\x1B[2J\x1B[1;1H");
    println!("Checking for upload/download folders...");
    let downloadFolder: &Path = Path::new("./Downloads");
    let uploadFolder: &Path = Path::new("./Uploads");
    FileSystem::make_dir_if_none_exist("./Downloads");
    FileSystem::make_dir_if_none_exist("./Uploads");

    println!("Trying connect to server...");

    Shared::NetworkUtils::fill_buffer_from_stream();
    let mut stream:TcpStream;
    loop{
         stream= match TcpStream::connect_timeout(&SocketAddr::new(IpAddr::from(Ipv4Addr::from_str("127.0.0.1").unwrap()), 2345), Duration::new(2, 0)){
            Ok(n)=>n,
            Err(e)=>{
                println!("Failed to connect!");
                println!("Retrying...");
                continue;
            },
        };
            break;
    }
    print!("\x1B[2J\x1B[1;1H");
    println!("Connected to server!\n");



    loop {
        println!("Enter your command:");

        let mut input = String::new();
        let command_size = io::stdin().read_line(&mut input).unwrap();

        input = String::from(input.trim_end_matches("\n").trim().to_lowercase());




        if input == "clear" {
            print!("\x1B[2J\x1B[1;1H");
        }

        if input == "ls" || input == "list" {
            CommandHandler::list_command(uploadFolder);

        }

        if input == "send" {
            let mut param_buffer = String::new();
            io::stdin().read_line(&mut param_buffer).unwrap();
            param_buffer = String::from(param_buffer.trim_end_matches("\n").trim().to_lowercase());

            stream.write(param_buffer.as_bytes());
        }


    }


}
