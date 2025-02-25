use crate::reader::{u8s_to_string, PDFReader, StreamReader};
use crate::structure::{AnyPDFData, Object, PDF};
use flate2::read::ZlibDecoder;
use regex::Regex;
use std::io;
use std::io::prelude::*;

impl AnyPDFData {
    fn consume_bool(reader: &mut PDFReader) -> Result<AnyPDFData, io::Error> {
        let v = match reader {
            t if t.peek(4) == "true".as_bytes() => {
                t.advance(4);
                true
            }
            t if t.peek(5) == "false".as_bytes() => {
                t.advance(5);
                false
            }
            &mut _ => unimplemented!(),
        };
        Ok(AnyPDFData::Boolean(v))
    }
    fn consume_num(reader: &mut PDFReader) -> Result<AnyPDFData, io::Error> {
        let mut num = String::new();
        loop {
            let token: char = reader.peek_next().into();
            if "-.0123456789+".contains(token) {
                num.push(token);
                reader.advance(1);
            } else {
                break;
            }
        }
        let i = num.parse::<i64>();
        let f = num.parse::<f64>();
        match (i, f) {
            (Ok(int), Ok(float)) => {
                if int as f64 == float {
                    Ok(Self::Integer(int))
                } else {
                    Ok(Self::Real(float))
                }
            }
            (Ok(int), _) => Ok(Self::Integer(int)),
            (_, Ok(float)) => Ok(Self::Real(float)),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid number")),
        }
    }
    fn consume_lit_str(reader: &mut PDFReader) -> Result<AnyPDFData, io::Error> {
        reader.advance(1);
        let mut level = 1;
        let mut string = String::new();
        loop {
            let mut token: char = reader.next().into();
            if token == '\\' {
                let next: char = reader.peek_next().into();
                // to add, to advance
                let push = match next {
                    'n' => ("\n", 1),
                    'r' => ("\r", 1),
                    'b' => ("\x7F", 1),
                    'f' => ("\x0C", 1),
                    '(' => ("(", 1),
                    ')' => (")", 1),
                    '\\' => ("\\", 1),
                    '\n' => ("", 1),
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' => {
                        let octal = u8::from_str_radix(u8s_to_string(reader.read(3))?, 8).unwrap();
                        static mut OCTAL_STR: String = String::new();
                        unsafe {
                            OCTAL_STR = char::from(octal).to_string();
                            #[allow(static_mut_refs)]
                            (OCTAL_STR.as_str(), 0)
                        }
                    }
                    _ => ("", 0),
                };
                reader.advance(push.1);
                string += push.0;
            } else {
                if token as u8 == 0x0D {
                    token = 0x0A as char;
                } else if token == '(' {
                    level += 1;
                } else if token == ')' {
                    level -= 1;
                    if level == 0 {
                        break;
                    }
                }
                string.push(token);
            }
        }
        return Ok(Self::String(string));
    }
    fn consume_hex_str(reader: &mut PDFReader) -> AnyPDFData {
        reader.advance(1);
        let mut string = String::new();
        loop {
            let token1: char = reader.next().into();
            let mut token2: char = reader.peek_next().into();
            if token1 == '>' {
                break;
            }
            if token2 == '>' {
                token2 = '0';
            } else {
                reader.advance(1);
            }
            string.push(
                char::from_u32(u32::from_str_radix(&format!("{}{}", token1, token2), 16).unwrap())
                    .unwrap(),
            );
        }
        return Self::String(string);
    }
    fn consume_name(reader: &mut PDFReader) -> Result<AnyPDFData, io::Error> {
        reader.advance(1);
        let mut name = String::new();
        let break_char = vec![
            u8::from_str_radix("00", 16).unwrap() as char,
            u8::from_str_radix("09", 16).unwrap() as char,
            u8::from_str_radix("0A", 16).unwrap() as char,
            u8::from_str_radix("0D", 16).unwrap() as char,
            u8::from_str_radix("0C", 16).unwrap() as char,
            u8::from_str_radix("20", 16).unwrap() as char,
            u8::from_str_radix("28", 16).unwrap() as char,
            u8::from_str_radix("29", 16).unwrap() as char,
            u8::from_str_radix("3C", 16).unwrap() as char,
            u8::from_str_radix("3E", 16).unwrap() as char,
            u8::from_str_radix("5B", 16).unwrap() as char,
            u8::from_str_radix("5D", 16).unwrap() as char,
            u8::from_str_radix("7B", 16).unwrap() as char,
            u8::from_str_radix("7D", 16).unwrap() as char,
            u8::from_str_radix("2F", 16).unwrap() as char,
            u8::from_str_radix("25", 16).unwrap() as char,
            '\\',
        ];
        loop {
            let token: char = reader.peek_next().into();
            if token == '#' {
                reader.advance(1);
                let hex = u8s_to_string(&reader.read(2))?;
                name.push(u8::from_str_radix(hex, 16).unwrap().into());
            } else if break_char.contains(&token) {
                break;
            } else {
                reader.advance(1);
                name.push(token)
            }
        }
        Ok(Self::Name(name))
    }
    fn consume_array(reader: &mut PDFReader) -> io::Result<AnyPDFData> {
        let mut objects: Vec<AnyPDFData> = vec![];
        reader.advance(1);
        loop {
            reader.skip_whitespace();
            let token: char = reader.peek_next().into();
            if token == ']' {
                reader.advance(1);
                break;
            }
            objects.push(Self::consume(reader)?);
        }
        Ok(Self::Array(objects))
    }
    fn consume_dict(reader: &mut PDFReader) -> Result<AnyPDFData, io::Error> {
        reader.advance(2);
        let mut dict = vec![];
        loop {
            reader.skip_whitespace();
            let peek = u8s_to_string(&reader.peek(2));
            if peek.is_ok_and(|x| x == ">>") {
                reader.advance(2);
                return Ok(Self::Dictionary(dict));
            }
            let Ok(AnyPDFData::Name(name)) = Self::consume_name(reader) else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Dictionary key must be a name",
                ));
            };
            reader.skip_whitespace();
            let value = Self::consume(reader)?;

            let index = dict.iter().position(|x: &(String, AnyPDFData)| x.0 == name);
            if let Some(index) = index {
                dict[index] = (name, value);
            } else {
                dict.push((name, value));
            }
        }
    }
    fn consume_objref(reader: &mut PDFReader) -> Option<AnyPDFData> {
        // assuming good
        let string = u8s_to_string(&reader.read_until(b'R')).unwrap(); // Clone the data to avoid borrow issues
        let numbers: Vec<i64> = string
            .split(' ')
            .filter_map(|x| x.parse::<i64>().ok())
            .collect();
        Some(Self::ObjRef(numbers[0], numbers[1]))
    }
    fn consume_stream(reader: &mut PDFReader, mapobj: AnyPDFData) -> Result<AnyPDFData, io::Error> {
        let AnyPDFData::Dictionary(dict) = mapobj else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Stream must be preceded by a dictionary",
            ));
        };
        let AnyPDFData::Integer(length) = dict
            .iter()
            .find(|x| x.0 == "Length")
            .expect("Stream must have a Length key")
            .1
        else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Stream must have a Length key",
            ));
        };
        reader.advance(6);
        if reader.peek_next() == b'\r' {
            reader.advance(1);
        }
        reader.advance(1);
        let stream: Vec<u8> = reader.read(length as usize).into();
        reader.skip_whitespace();
        reader.advance(9);
        Ok(AnyPDFData::Stream(dict, stream))
    }
    fn consume(reader: &mut PDFReader) -> Result<AnyPDFData, io::Error> {
        reader.skip_whitespace();
        match reader {
            t if t.peek(2) == "<<".as_bytes() => {
                let object = Self::consume_dict(t);
                t.skip_whitespace();
                if t.peek(6) == "stream".as_bytes() {
                    Self::consume_stream(t, object?)
                } else {
                    object
                }
            }
            t if t.peek(1) == "t".as_bytes() || t.peek(1) == "f".as_bytes() => {
                Self::consume_bool(t)
            }
            t if {
                let regex = Regex::new(r"^\d+\s+\d+\s+$").unwrap();
                let string = u8s_to_string(&t.peek_until(b'R')); // Clone the data to avoid borrow issues
                if string.is_err() {
                    false
                } else {
                    let unwrapped = string.unwrap();
                    regex.is_match(&unwrapped)
                }
            } =>
            {
                Ok(Self::consume_objref(t).unwrap())
            }
            t if "-.0123456789+".contains(t.peek_next() as char) => Self::consume_num(t),
            t if t.peek(1) == "(".as_bytes() => Self::consume_lit_str(t),
            t if t.peek(1) == "<".as_bytes() => Ok(Self::consume_hex_str(t)),
            t if t.peek(1) == "/".as_bytes() => Self::consume_name(t),
            t if t.peek(1) == "[".as_bytes() => Self::consume_array(t),
            _t => {
                // dbg!(u8s_to_string(&_t.clone().peek(100)));
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unable to parse object",
                ))
            }
        }
    }
}

pub fn parse(reader: &mut PDFReader) -> Result<PDF, io::Error> {
    let mut pdf = PDF {
        version: u8s_to_string(&reader.read_until('\n' as u8))?.to_string(),
        objects: vec![],
    };
    let obj_beginning = Regex::new(r"^\d+\s+\d+\s+obj").unwrap();
    while !reader.at_eof() {
        // dbg!(reader.bytes.len() - reader.offset);
        // dbg!(reader.offset, u8s_to_string(&reader.peek(100)));
        let line = u8s_to_string(&reader.read_until('\n' as u8));
        if line.is_err() {
            continue;
        }
        let line = line.unwrap();
        if obj_beginning.is_match(&line) {
            let obj_info = obj_beginning
                .find(&line)
                .unwrap()
                .as_str()
                .split_whitespace()
                .filter_map(|x| x.parse::<i64>().ok())
                .collect::<Vec<_>>();
            let object = Object {
                number: obj_info[0],
                gen: obj_info[1],
                data: AnyPDFData::consume(reader)?,
            };
            if let AnyPDFData::Stream(data, values) = &object.data {
                fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
                    let mut deflater = ZlibDecoder::new(&bytes[..]);
                    let mut s = String::new();
                    deflater.read_to_string(&mut s)?;
                    Ok(s)
                }
                let result = decode_reader(values.clone());
                match result {
                    Ok(x) => {
                        dbg!(x);
                        ()
                    }
                    Err(x) => {
                        // dbg!(data);
                        ()
                    }
                }
            }
            pdf.objects.push(object.clone());
        }
    }
    Ok(pdf)
}
