extern crate blorb;
extern crate glulx;

use std::env::args;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use blorb::{
    BlorbCursor,
    Chunk,
    Usage,
};

mod machine;


type Result<T> = std::result::Result<T, Box<Error + Send + Sync>>;


fn run_blorb(exec: Chunk, blorb: BlorbCursor<File>) -> Result<i32> {
    use blorb::Chunk::*;
    match exec {
        Adrift{..}
                | AdvSys{..}
                | Agt{..}
                | Alan{..}
                | Exec{..}
                | Hugo{..}
                | Level9{..}
                | MagneticScrolls{..}
                | Tads2{..}
                | Tads3{..}
                | ZCode{..} => {
            println!("unsupported game type");
            Ok(1)
        },
        Unknown{..} => {
            println!("unrecognized chunk");
            Ok(1)
        },
        Glulx{code} => {
            machine::glulx::run_blorb(code, blorb);
            Ok(0)
        },
        _ => {
            println!("non-executable chunk found");
            Ok(1)
        }
    }
}


fn run(file: String) -> Result<i32> {
    let file = Path::new(&file);


    if let Some("ulx") = file.extension().and_then(|x|x.to_str()) {
        File::open(file)
            .map_err(|err| err.into())
            .and_then(|mut rom| {
                let mut code = Vec::new();
                rom.read_to_end(&mut code)?;
                machine::glulx::run(code);
                Ok(0)
            })
    } else {
        File::open(file)
            .and_then(|rom| BlorbCursor::from_file(rom))
            .and_then(|mut blorb| blorb.load_resource(Usage::Exec, 0)
                .map(|chunk| (chunk, blorb)))
            .map_err(|err| err.into())
            .and_then(|(chunk, blorb)| run_blorb(chunk, blorb))
    }
}


fn main() {
    match args().nth(1)
            .ok_or_else(||"no file provided".into())
            .and_then(run) {
        Ok(ret) => {
            println!("finished running");
            std::process::exit(ret)
        },
        Err(ref err) => {
            println!("error occured: {}", err);
            std::process::exit(1)
        },
    }
}
