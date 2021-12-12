use std::{str::FromStr, fmt::Display};

#[derive(PartialEq, PartialOrd)]
pub struct ChunkType {

}
impl FromStr for ChunkType {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!() ;
    }

}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str; 

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl ChunkType {

    pub fn bytes(&self) -> [u8; 4] {
        unimplemented!();
    }
    pub fn is_valid(&self) -> bool {
        unimplemented!();
    }

    pub fn is_critical(&self) -> bool {
        unimplemented!();
    }

    pub fn is_public(&self) -> bool {
        unimplemented!();
    }
    
    pub fn is_reserved_bit_valid(&self) -> bool {
        unimplemented!();
    }

    pub fn is_safe_to_copy(&self) -> bool {
        unimplemented!();
    }


}