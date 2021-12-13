use std::fmt::{Display};



use crate::chunk_types::ChunkType;

pub struct Chunk {
    chunk_data: Vec<u8>,
    chunk_type: ChunkType,
    crc: u32,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // first data length
        // then type of chunk
        // then message itself
        // crc at last
        // crc doesn't match invalid message.

        // conversion is possible with to_be_bytes and from_be_bytes
        
        //get the length of message
        if value.len() < 12 {
            return Err("Something went wrong");
        }
        let (u32_message_length_bytes, rest) = value.split_at(std::mem::size_of::<u32>());
        let message_length = u32::from_be_bytes(u32_message_length_bytes.try_into().unwrap());
        let (ctb,message_and_crc) = rest.split_at(4);
        let chunk_type = ChunkType::try_from([ctb[0],ctb[1],ctb[2],ctb[3]]).unwrap();
        let (message,crc_bytes) = message_and_crc.split_at(message_length as usize);
        // println!("{}",String::from_utf8(message.to_vec()).unwrap());
        let crc = u32::from_be_bytes(crc_bytes.try_into().unwrap());
        let crc_type_message_data: Vec<u8> = ctb.iter().chain(message.iter()).copied().collect() ;
        let compute_crc = crc32fast::hash(&crc_type_message_data);
        if compute_crc != crc {
            return Err("CRC doesn't match. Message must be different");
        }

        Ok(
            Self {
                chunk_data: message.to_vec(),
                chunk_type: chunk_type,
                crc: crc
            }
        )
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = String::from_utf8(self.chunk_data.to_owned());
        write!(f,"{}",data.unwrap())
    }
}

impl Chunk {
    pub fn length(&self) -> u32 {
         self.chunk_data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        self.chunk_data.as_slice()
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String,&'static str> {
        match String::from_utf8(self.chunk_data.clone()) {
            Ok(res) => Ok(res),
            Err(_) => Err("Couldn't convert to string from utf8")
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.chunk_data.clone()
    }
}