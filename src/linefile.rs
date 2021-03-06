// See https://remarkablewiki.com/tech/filesystem

use std::convert::TryFrom;
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

trait ParseFrom
where
    Self: std::marker::Sized,
{
    // Read the number of items, then parse a vector
    fn parse_from(version: i32, bytes: &mut Bytes<BufReader<File>>) -> Result<Self, ParseError>;
}

#[derive(Debug)]
pub struct LineFile {
    pub version: i32,
    pub layers: Vec<Layer>,
}

#[derive(Debug)]
pub struct Layer {
    pub strokes: Vec<Stroke>,
}

impl ParseFrom for Layer {
    fn parse_from(version: i32, bytes: &mut Bytes<BufReader<File>>) -> Result<Layer, ParseError> {
        let strokes = parse_multiple(version, bytes)?;

        Ok(Layer { strokes })
    }
}

#[derive(Debug)]
pub enum Pen {
    // Based on https://github.com/ax3l/lines-are-rusty/blob/develop/src/lib.rs
    BallPoint,
    Marker,
    Fineliner,
    Pencil,
    MechanicalPencil,
    PaintBrush,
    Highlighter,
    Eraser,
    EraseArea,
    EraseAll,
    Calligraphy,
    Pen,
    SelectionBrush,
}

impl TryFrom<u32> for Pen {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Pen, ParseError> {
        // Copied from https://github.com/ax3l/lines-are-rusty/blob/develop/src/lib.rs
        match value {
            0 => Ok(Pen::PaintBrush),
            1 => Ok(Pen::Pencil),
            2 => Ok(Pen::Pen),
            3 => Ok(Pen::Marker),
            4 => Ok(Pen::Fineliner),
            5 => Ok(Pen::Highlighter),
            6 => Ok(Pen::Eraser),
            7 => Ok(Pen::MechanicalPencil),
            8 => Ok(Pen::EraseArea),
            9 => Ok(Pen::EraseAll),
            10 => Ok(Pen::SelectionBrush),
            11 => Ok(Pen::SelectionBrush),
            12 => Ok(Pen::PaintBrush),
            13 => Ok(Pen::MechanicalPencil),
            14 => Ok(Pen::Pencil),
            15 => Ok(Pen::BallPoint),
            16 => Ok(Pen::Marker),
            17 => Ok(Pen::Fineliner),
            18 => Ok(Pen::Highlighter),
            21 => Ok(Pen::Calligraphy),
            _ => Err(ParseError::new(&format!(
                "Invalid value for Pen: {}",
                value
            ))),
        }
    }
}

#[derive(Debug)]
pub enum Color {
    Black,
    Gray,
    White,
}

impl TryFrom<u32> for Color {
    type Error = ParseError;

    fn try_from(value: u32) -> Result<Color, ParseError> {
        match value {
            0 => Ok(Color::Black),
            1 => Ok(Color::Gray),
            2 => Ok(Color::White),
            _ => Err(ParseError::new(&format!(
                "Invalid value for Color: {}",
                value
            ))),
        }
    }
}

#[derive(Debug)]
pub struct Stroke {
    pub pen: Pen,
    pub color: Color,
    pub width: f32,
    pub segments: Vec<Segment>,
}

impl ParseFrom for Stroke {
    fn parse_from(version: i32, bytes: &mut Bytes<BufReader<File>>) -> Result<Stroke, ParseError> {
        let pen = Pen::try_from(parse_u32(bytes)?)?;
        let color = Color::try_from(parse_u32(bytes)?)?;
        discard_bytes(bytes, 4)?;
        let width = parse_f32(bytes)?;
        if version >= 5 {
            discard_bytes(bytes, 4)?;
        }
        let segments = parse_multiple(version, bytes)?;

        Ok(Stroke {
            pen,
            color,
            width,
            segments,
        })
    }
}

#[derive(Debug)]
pub struct Segment {
    // According to https://plasma.ninja/blog/devices/remarkable/binary/format/2017/12/26/reMarkable-lines-file-format.html
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub direction: f32,
    pub width: f32,
    pub pressure: f32,
}

impl ParseFrom for Segment {
    fn parse_from(
        _version: i32,
        bytes: &mut Bytes<BufReader<File>>,
    ) -> Result<Segment, ParseError> {
        let x = parse_f32(bytes)?;
        let y = parse_f32(bytes)?;
        let speed = parse_f32(bytes)?;
        let direction = parse_f32(bytes)?;
        let width = parse_f32(bytes)?;
        let pressure = parse_f32(bytes)?;

        let segment = Segment {
            x,
            y,
            speed,
            direction,
            width,
            pressure,
        };

        Ok(segment)
    }
}

fn parse_multiple<T: ParseFrom>(
    version: i32,
    bytes: &mut Bytes<BufReader<File>>,
) -> Result<Vec<T>, ParseError> {
    let count = parse_u32(bytes)?;
    let mut items: Vec<T> = vec![];
    for _ in 0..count {
        let item = T::parse_from(version, bytes)?;
        items.push(item);
    }

    Ok(items)
}

impl LineFile {
    pub fn parse(filename: &str) -> Result<LineFile, ParseError> {
        let file = File::open(filename)?;

        let reader = BufReader::new(file);
        let bytes = &mut reader.bytes();

        parse_header(bytes)?;

        let version = parse_version(bytes)?;

        parse_string(bytes, 10)?; // Chomp extra bytes

        let layers: Vec<Layer> = parse_multiple(version, bytes)?;

        Ok(LineFile { version, layers })
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

fn discard_bytes(bytes: &mut Bytes<BufReader<File>>, count: i32) -> Result<(), ParseError> {
    for _ in 0..count {
        match bytes.next() {
            None => {
                return Err(ParseError::new(&format!(
                    "Unexpected end of file while discarding {} bytes",
                    count
                )));
            }
            Some(_) => {}
        }
    }

    Ok(())
}

fn parse_string(bytes: &mut Bytes<BufReader<File>>, count: i32) -> Result<String, ParseError> {
    let mut buffer: Vec<u8> = vec![];

    for _ in 0..count {
        match bytes.next() {
            None => {
                return Err(ParseError::new(&format!(
                    "Unexpected end of file while parsing string of length {}",
                    count
                )));
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
    let mut buffer: [u8; 4] = [0; 4];

    for i in 0..4 {
        match bytes.next() {
            None => {
                return Err(ParseError::new("Unexpected end of file while parsing u32"));
            }
            Some(byte) => {
                buffer[i] = byte?;
            }
        }
    }

    // Little-endian
    Ok(u32::from_le_bytes(buffer))
}

fn parse_f32(bytes: &mut Bytes<BufReader<File>>) -> Result<f32, ParseError> {
    let mut buffer: [u8; 4] = [0; 4];

    for i in 0..4 {
        match bytes.next() {
            None => {
                return Err(ParseError::new("Unexpected end of file while parsing f32"));
            }
            Some(byte) => {
                buffer[i] = byte?;
            }
        }
    }

    Ok(f32::from_le_bytes(buffer))
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
