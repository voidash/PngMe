use std::{str::FromStr, fmt::Display};

#[derive(PartialEq, PartialOrd,Debug)]
pub struct ChunkType {
    data_length: [u8;4],
}

impl FromStr for ChunkType {

    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, &'static str> {
        if s.len() != 4  {
            return Err("Chunk Length should be of length 4.");
        }
        let value = s.as_bytes();
        for &i in value {
            if (i < b'a' && i >= b'z') || (i < b'A' && i >= b'Z') {
                return Err("invalid bytes. uppercase letter 'A' starts from 65, you provided invalid data.");
            }
        }
        let sized: [u8; 4] = [value[0],value[1],value[2],value[3]];
        Ok(Self::try_from(sized).unwrap())
    }


}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.data_length).unwrap();
        write!(f, "{}",s)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str; 

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for i in value {
            if i < 65 {
                return Err("invalid bytes. uppercase letter 'A' starts from 65, you provided invalid data.");
            }
        }
        Ok(Self {
            data_length: value
        })
    }
}
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.data_length
    }

    pub fn is_valid(&self) -> bool {
        for i in self.data_length {
            if (i < b'a' && i >= b'z') || (i < b'A' && i >= b'Z') {
                return false;
            }
        }
        if self.data_length[2] >= 97 {
            return false;
        }
        true
    }

    pub fn is_critical(&self) -> bool {
        // 2^5 is 32
        // ancillary bit or 5th bit of a byte makes a difference between uppercase letters and lowercase letters

         self.data_length[0] < 97 
    }

    pub fn is_public(&self) -> bool {
        // uppercase ASCII means its public
        self.data_length[1] < 97
    }
    
    pub fn is_reserved_bit_valid(&self) -> bool {
        //uppercase third letters
        self.data_length[2] < 97
    }

    pub fn is_safe_to_copy(&self) -> bool {
        //lowercase means its safe to copy
        self.data_length[3] >= 97
    }


}