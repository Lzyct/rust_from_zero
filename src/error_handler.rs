use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

pub fn error_handler() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    // create the file if not exist
                    match File::create("hello.txt") {
                        Ok(fc) => fc,
                        Err(e) => {
                            panic!("There was a problem creating the file: {:?}", e)
                        }
                    }
                }
                _ => {
                    panic!("There was a problem opening the file: {:?}", error)
                }
            }
        }
    };

    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("There was a problem creating the file: {:?}", error)
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error)
        }
    });

    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    println!("f: {:?}", f);

    read_name_from_file().expect("TODO: panic message");
}

fn read_name_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}