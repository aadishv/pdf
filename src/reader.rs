use std::io::Read;
use std::*;

pub fn u8s_to_string(bytes: &[u8]) -> Result<&str, io::Error> {
    str::from_utf8(bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub trait StreamReader: Clone + From<fs::File> + From<String> {
    fn advance(&mut self, amt: usize);
    fn read(&mut self, amt: usize) -> &[u8];
    fn at_eof(&self) -> bool;
    fn peek(&self, amt: usize) -> &[u8];
    fn peek_next(&self) -> u8;
    fn next(&mut self) -> u8;
    fn skip_whitespace(&mut self);
    fn peek_until(&self, byte: u8) -> &[u8];
    fn read_until(&mut self, byte: u8) -> &[u8];
}

pub struct PDFReader {
    pub bytes: &'static [u8],
    pub offset: usize,
}

impl From<fs::File> for PDFReader {
    fn from(file: fs::File) -> Self {
        let buf: Vec<u8> = file
            .try_clone()
            .unwrap()
            .bytes()
            .filter_map(Result::ok)
            .collect();
        Self {
            bytes: Box::leak(buf.into_boxed_slice()),
            offset: 0,
        }
    }
}

impl From<String> for PDFReader {
    fn from(s: String) -> Self {
        Self {
            bytes: Box::leak(s.into_bytes().into_boxed_slice()),
            offset: 0,
        }
    }
}
impl Clone for PDFReader {
    fn clone(&self) -> Self {
        PDFReader {
            bytes: self.bytes,
            offset: self.offset,
        }
    }
}

impl StreamReader for PDFReader {
    fn advance(&mut self, amt: usize) {
        self.offset = (self.offset + amt).min(self.bytes.len() - 1);
    }
    fn at_eof(&self) -> bool {
        self.offset >= self.bytes.len() - 1
    }
    fn read(&mut self, amt: usize) -> &[u8] {
        let start = self.offset;
        self.advance(amt);
        &self.bytes[start..self.offset]
    }
    fn peek(&self, amt: usize) -> &[u8] {
        &self.bytes[self.offset..(self.offset + amt).min(self.bytes.len())]
    }
    fn peek_next(&self) -> u8 {
        self.bytes[self.offset]
    }
    fn next(&mut self) -> u8 {
        let next = self.bytes[self.offset];
        self.advance(1);
        next
    }
    fn skip_whitespace(&mut self) {
        while self.peek_next().is_ascii_whitespace() {
            self.advance(1);
        }
    }
    fn peek_until(&self, byte: u8) -> &[u8] {
        let end = (self.offset..self.bytes.len())
            .find(|&i| self.bytes[i] == byte)
            .unwrap_or(self.offset);
        &self.bytes[self.offset..end]
    }
    fn read_until(&mut self, byte: u8) -> &[u8] {
        let start = self.offset;
        let end = (self.offset..self.bytes.len())
            .find(|&i| self.bytes[i] == byte)
            .unwrap_or(self.offset);
        self.offset = (end + 1).min(self.bytes.len());
        &self.bytes[start..end]
    }
}
