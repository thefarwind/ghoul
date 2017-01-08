extern crate blorb;
extern crate glulx;

use std::env::args;
use std::fs::File;
use std::error::Error;

use blorb::{
    BlorbCursor,
    Chunk,
    Usage,
};

mod machine;


type Result<T> = std::result::Result<T, Box<Error + Send + Sync>>;


fn run(file: String) -> Result<i32> {
    use blorb::Chunk::*;

    let (blorb, exec) = File::open(file)
        .and_then(|rom| BlorbCursor::from_file(rom))
        .and_then(|mut blorb| blorb.load_resource(Usage::Exec, 0)
            .map(|chunk| (blorb, chunk)))?;

    match exec {
        Adrift{code} => println!("got code!"),
        AdvSys{code} => println!("got code!"),
        Agt{code} => println!("got code!"),
        Alan{code} => println!("got code!"),
        Exec{code} => println!("got code!"),
        Glulx{code} => machine::glulx::run(code, blorb),
        Hugo{code} => println!("got code!"),
        Level9{code} => println!("got code!"),
        MagneticScrolls{code} => println!("got code!"),
        Tads2{code} => println!("got code!"),
        Tads3{code} => println!("got code!"),
        ZCode{code} => println!("got code!"),
        Unknown{..} => println!("unrecognized chunk"),
        _ => println!("non-executable chunk found"),
    }
    Ok(0)
}


fn main() {
    match args().nth(1) {
        Some(file) => {
            match run(file) {
                Err(ref err) => {
                    println!("error occured: {}", err);
                    std::process::exit(1)
                },
                Ok(ret) => {
                    println!("finished running");
                    std::process::exit(ret)
                },
            }
        },
        None => {
            println!("no file provided");
            std::process::exit(1)
        }
    }
}
