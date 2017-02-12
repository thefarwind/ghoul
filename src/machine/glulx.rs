use std::io::{
    Read,
    Seek,
};

use blorb::BlorbCursor;
use glulx::Glulx;


pub fn run_blorb<T: Read + Seek>(code: Vec<u8>, blorb: BlorbCursor<T>) {
    let mut glulx = Glulx::from_rom(code).unwrap();
    glulx.run();
}

pub fn run(code: Vec<u8>) {
    let mut glulx = Glulx::from_rom(code).unwrap();
    glulx.run();
}
