use std::fs;

pub fn make_dir_if_none_exist(folderPath:&str){
    let folderName:&str = folderPath.split('/').last().unwrap();
    loop{
        match fs::read_dir(folderPath){
            Ok(n)=>{
                println!("Found {folderName} folder!");
                break;
            },
            Err(e)=>{
                println!("No {folderName} folder found, trying to make one...");
            },
        };
        match fs::create_dir(folderPath){
            Ok(n)=>{
                println!("Made a {folderName} folder!");
                break;
            },
            Err(e)=>{
                println!("Error making {folderName} folder: {:?}", e);
                continue;
            },
        };

    }
}