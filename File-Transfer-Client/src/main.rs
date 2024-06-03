mod FileSystem;

use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::ptr::null;
use std::str::FromStr;
use std::{fs, io, thread};
use std::fs::{File, ReadDir};
use std::ops::Deref;
use std::path::Component::CurDir;
use std::path::Path;
use std::time::Duration;

fn main() {
    println!("Booting up...");

    println!("Checking for upload/download folders...");
    let downloadFolder: &str = "./Downloads/";
    let uploadFolder: &str = "./Uploads/";
    FileSystem::make_dir_if_none_exist("./Downloads");
    FileSystem::make_dir_if_none_exist("./Uploads");
    println!("Trying connect to server...");

    let mut has_server_connection:bool = false;
    loop{
        let mut stream = match TcpStream::connect_timeout(&SocketAddr::new(IpAddr::from(Ipv4Addr::from_str("127.0.0.1").unwrap()), 2345), Duration::new(1, 0)){
            Ok(n)=>{
                has_server_connection = true;
            },
            Err(e)=>(),
        };

        if  has_server_connection {
            break;
        }
        println!("Failed to connect!");
        println!("Retrying...");
    }
    print!("\x1B[2J\x1B[1;1H");
    println!("Connected to server!\n");



    loop {
        println!("Enter your command:");

        let mut input = String::new();
        let commandSize = io::stdin().read_line(&mut input).unwrap();

        input = String::from(input.trim_end_matches("\n").trim().to_lowercase());




        if input == "clear" {
            print!("\x1B[2J\x1B[1;1H");
        }

        if input == "ls" || input == "list" {
            println!("\nFiles in upload folder:");
            let mut dirList = fs::read_dir(uploadFolder).unwrap();

            if dirList.by_ref().count() == 0{
                println!("Uploads folder is empty put something u wanna upload in it!")
            }

            for entry in dirList.by_ref(){
                println!("Bla");
                println!("- {:?}", entry.unwrap().file_name().into_string().unwrap())
            }
            print!("\n");

        }
    }


}
