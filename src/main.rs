#![feature(type_alias_impl_trait)]
#![feature(bufreader_peek)]
mod reader;
use crate::reader::*;
// imports
mod parser;
mod structure;
#[cfg(test)]
mod test;
use parser::parse;
use std::io;

const BLOCK: bool = true;
fn main() -> io::Result<()> {
    // let test = PDFTEST.to_owned();
    let test = std::fs::File::open("src/test.pdf").unwrap();
    let mut reader = PDFReader::from(test);
    let pdf = parse(&mut reader);
    // dbg!(pdf);
    if BLOCK {
        Err(io::Error::new(io::ErrorKind::Other, "Blocking"))
    } else {
        Ok(())
    }
}
