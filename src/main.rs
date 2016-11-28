use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::BufReader;
use std::io;
use std::env;


fn main() {
    // Create a path to the desired file
    let path = Path::new("/path/to/email/ids.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    match print_email(file) {
        Err(err) => println!("{}", err.description()),
        _ => (),
    }
    // `file` goes out of scope, and the "hello.txt" file gets closed
}



fn print_email(file: File) -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Ok(())
    }
    let start = args[1].to_string();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = try!(line);
        let lowercase_line = line.to_lowercase();
        if lowercase_line.starts_with(&start) {
            println!("{}",line)
        }
    }
    Ok(())
}
