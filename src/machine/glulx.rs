use std::io::{
    Read,
    Seek,
};

use blorb::BlorbCursor;
use glulx::Glulx;


pub fn run<T: Read + Seek>(code: Vec<u8>, blorb: BlorbCursor<T>) {
    let glulx = Glulx::from_rom(code);
    println!("running glulx code!");
}
