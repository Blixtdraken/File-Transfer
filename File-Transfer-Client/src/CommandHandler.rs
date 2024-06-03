use std::fs::ReadDir;
use std::path::Path;

pub fn list_command(folder:&Path){
    println!("\nFiles in upload folder:\n");
    let mut dir_list:ReadDir = folder.read_dir().unwrap();

    let mut dir_empty = true;
    for entry in dir_list {
        dir_empty = false;
        let is_file:bool = entry.as_ref().unwrap().file_type().unwrap().is_file();
        if(is_file){
            let file = entry.unwrap();
            println!("- {}", file.file_name().into_string().unwrap())
        }

    }

    if dir_empty{
        println!("Uploads folder is empty put something u wanna upload in it!")
    }


    print!("\n");
}