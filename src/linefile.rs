use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::prelude::Read;
use std::io::{BufReader, Bytes};
use std::{i32, str};

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl ParseError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::convert::From<std::io::Error> for ParseError {
    fn from(other: std::io::Error) -> Self {
        Self::new(&format!("{}", other))
    }
}

impl std::convert::From<std::str::Utf8Error> for ParseError {
    fn from(other: std::str::Utf8Error) -> Self {
        Self::new(&format!("{}", other))
    }
}

impl std::convert::From<std::num::ParseIntError> for ParseError {
    fn from(other: std::num::ParseIntError) -> Self {
        Self::new(&format!("{}", other))
    }
}

#[derive(Debug)]
pub struct LineFile {
    version: i32,
    num_layers: u32,
}

impl LineFile {
    pub fn parse(filename: &str) -> Result<LineFile, ParseError> {
        let file = File::open(filename)?;

        let reader = BufReader::new(file);
        let bytes = &mut reader.bytes();

        parse_header(bytes)?;

        let version = parse_version(bytes)?;

        parse_string(bytes, 10)?; // Chomp extra bytes

        let num_layers = parse_u32(bytes)?;

        Ok(LineFile {
            version,
            num_layers,
        })
    }
}

fn parse_header(bytes: &mut Bytes<BufReader<File>>) -> Result<(), ParseError> {
    let header = parse_string(bytes, 32)?;
    if header == "reMarkable .lines file, version=" {
        Ok(())
    } else {
        Err(ParseError::new(&format!("Invalid header: '{}'", header)))
    }
}

fn parse_string(bytes: &mut Bytes<BufReader<File>>, count: i32) -> Result<String, ParseError> {
    let mut buffer: Vec<u8> = vec![];

    for _ in 0..count {
        match bytes.next() {
            None => {
                return Err(ParseError::new("Unexpected end of file"));
            }
            Some(byte) => {
                let byte = byte?;
                buffer.push(byte);
            }
        }
    }

    let string = str::from_utf8(&buffer)?;

    Ok(string.to_owned())
}

fn parse_u32(bytes: &mut Bytes<BufReader<File>>) -> Result<u32, ParseError> {
    let mut buffer: [u32; 4] = [0; 4];

    for i in 0..4 {
        match bytes.next() {
            None => {
                return Err(ParseError::new("Unexpected end of file"));
            }
            Some(byte) => {
                buffer[i] = byte? as u32;
            }
        }
    }

    // Little-endian
    Ok(buffer[0] + (buffer[1] << 8) + (buffer[2] << 16) + (buffer[3] << 24))
}

fn parse_version(bytes: &mut Bytes<BufReader<File>>) -> Result<i32, ParseError> {
    let version_string = parse_string(bytes, 1)?;
    let version: i32 = version_string.parse()?;

    if version >= 3 {
        Ok(version)
    } else {
        Err(ParseError::new(
            "Invalid line file version. Version 3 or higher required.",
        ))
    }
}
