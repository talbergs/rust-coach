use std::env;
use std::io::*;

fn main() {
    let argument = env::args().skip(1).next();
    let mut input = Vec::new();

    match argument {
        Some(v) => {
            let c = v.as_bytes();

            c.reverse();
            println!("{}", String::from_utf8_lossy(c));
        }
        None => {
            // it's ok
        }
    }

    match stdin().read_to_end(&mut input) {
        Ok(_) => {
            input.reverse();
            print!("{}", String::from_utf8_lossy(&input));
        }
        Err(error) => println!("error: {}", error),
    }
}
