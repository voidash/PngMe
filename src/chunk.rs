use crate::chunk_type::ChunkType;

pub struct Chunk {
    chunk_data: Vec<u32>
}

impl Chunk {
    pub fn length(&self) -> u32 {

    }

    pub fn chunk_type(&self) -> &ChunkType {

    }
    pub fn data(&self) -> &[u8] {

    }

    pub fn crc(&self) -> u32 {

    }

    pub fn data_as_string(&self) -> Result<String> {

    }

    pub fn as_bytes(&self) -> Vec<u8> {

    }
}