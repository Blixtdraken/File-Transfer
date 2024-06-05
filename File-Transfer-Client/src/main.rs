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

    'connect: loop {
        println!("Trying connect to server...");


        let mut stream:TcpStream;
        'main: loop{
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
        println!("Connected to {}\n",stream.peer_addr().unwrap().to_string());



        loop {
            println!("Enter your command:");

            let mut input = String::new();
            let command_size = io::stdin().read_line(&mut input).unwrap();

            let mut send_buffer = [0u8;256];

            input = String::from(input.trim_end_matches("\n").trim().to_lowercase());




            if input == "clear" {
                print!("\x1B[2J\x1B[1;1H");
                continue;
            }

            if input == "ls" || input == "list" {
                CommandHandler::list_command(uploadFolder);
                continue;
            }

            if input == "send" {
                let mut param_buffer = String::new();
                io::stdin().read_line(&mut param_buffer).unwrap();
                param_buffer = String::from(param_buffer.trim_end_matches("\n").trim().to_lowercase());
                Shared::NetworkUtils::place_bytes_in_buffer(param_buffer.as_bytes(), &mut send_buffer);
                stream.write(&send_buffer);
                continue;
            }

            println!("\x1b[93mEnter a valid command!\x1b[0m\n")


        }
    }



}
