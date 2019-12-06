use std::fs::{remove_file, File, read_to_string};
use std::io::ErrorKind;
use std::io::{Read, Write, Error as ioError};
use whoami;

fn main() {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {:?}", e),
            },
            other_error => panic!("Problem opening file: {:?}", other_error)
        },
    };

    writeln!(&mut f, "{}", whoami::username()).expect("Error writing to file");

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating file: {:?}", error)
            })
        } else {
            panic!("Problem opening file: {:?}", error)
        }
    });

    let s = read_username_from_file().expect("Error getting username");
    print!("{}", s);

    let s = concise_read_username_from_file().expect("Error getting username");
    print!("{}", s);

    let s = even_more_concise_read_username_from_file().expect("Error getting username");
    print!("{}", s);

    let s = the_most_concise_read_username_from_file().expect("Error getting username");
    print!("{}", s);

    remove_file("hello.txt").expect("Error while deleting file");
}

fn read_username_from_file() -> Result<String, ioError> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn concise_read_username_from_file() -> Result<String, ioError> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn even_more_concise_read_username_from_file() -> Result<String, ioError> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn the_most_concise_read_username_from_file() -> Result<String, ioError> {
    read_to_string("hello.txt")
}
