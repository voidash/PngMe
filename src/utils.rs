use crate::png::{Png, self};
use crate::chunk::Chunk;
use crate::chunk_types::ChunkType;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use crc32fast::hash;

pub fn read_file_and_get_bytes(filename: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();

    let f = File::open(filename).unwrap();
    for byte in f.bytes() 
    {
        bytes.push(byte.unwrap());
    }
    bytes
}
pub fn write_to_file(filename: &str, bytes: Vec<u8>) {
    std::fs::write(filename, bytes).unwrap();
}

pub fn encode(filename: &str, message: &str, bit_type: &str) {
    let bytes = read_file_and_get_bytes(filename);
    let mut png_file = Png::try_from(bytes.as_ref()).unwrap();

    let combined_type_and_message: Vec<u8> = bit_type.as_bytes()
    .iter()
    .chain(message.as_bytes().iter())
    .copied()
    .collect();

    let crc =  hash(
        &combined_type_and_message
    );

    let new_chunk = Chunk::new(
        message.as_bytes(),ChunkType::from_str(bit_type).unwrap(), crc
    );

    png_file.append_chunk(new_chunk);
    write_to_file(filename,png_file.as_bytes());
}

pub fn decode(filename:&str, bit_type: &str) -> String {
    let bytes = read_file_and_get_bytes(filename);
    let png_file = Png::try_from(bytes.as_ref()).unwrap();
    let mut mtd = String::new();
    if let Some(f) = png_file.chunk_by_type(bit_type) {
            mtd = String::from(f.data_as_string().unwrap());
            println!("{}",mtd);
    }
    mtd
}