use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn main(){
    let mut list = File::open("task.txt").unwrap();
            let mut file = String::new();
                list.read_to_string(&mut file).unwrap();
                    println!("Previously added Tasks! \n{file}");
         
         println!("What are your Goals Today ?"); 
         let mut counter = 0;
    
    loop{
            let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("Wrong input");
                         if input == "\\q\n" {
                            break
                         } 
                    counter += 1;
                let mut file = OpenOptions::new().read(true)
                .write(true).append(true).create(false)
                .open("task.txt").unwrap();

        writeln!(file,"{}: {}", &counter ,&input)
       .expect("Failed to write task on file");
    }
    println!("Have a Great Day! Keep Grinding");

}































/*
pub fn add(value: &String, key: &i32 ) {

    let mut file = OpenOptions::new().read(true).write(true).append(true).create(false).open("task.txt").unwrap();

    writeln!(file,"{}: {}", key, value).expect("Failed to write task on file");
    println!("Task Saved!\n{}: {}", key, value);

}

*/
