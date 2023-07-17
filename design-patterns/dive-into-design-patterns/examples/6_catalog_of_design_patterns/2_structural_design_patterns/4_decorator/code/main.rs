use std::io::{BufReader, Cursor, Read};

/// cargo r --example decorator
fn main() {
    let mut buf = [0u8; 10];

    // A buffered reader decorates a vector reader which wraps input data.
    let mut input = BufReader::new(Cursor::new("Input data, > 10"));

    input.read(&mut buf).ok();

    print!("Read from a buffered reader: ");

    for byte in buf {
        print!("{}", char::from(byte));
    }

    println!(); // Read from a buffered reader: Input data
}
