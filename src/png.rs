
use crate::chunk::Chunk;
pub struct Png {
    header: [u8;8],
    chunks: Vec<Chunk>,
}
impl TryFrom<&[u8]> for Png {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
         let mut chunks : Vec<Chunk> = Vec::new();
         let mut copy_value = value.to_owned().into_iter().peekable();
         let standard_header: Vec<_> = copy_value.by_ref().take(8).collect();
         if standard_header.as_slice() != Png::STANDARD_HEADER {
             return Err("Not a PNG file");
         }

         // collection of chunks? 
         while copy_value.peek().is_some() {
             let chunk_length_bytes : Vec<_> = copy_value.by_ref().take(4).collect();
             let len_chunk_usize = usize::from_be_bytes(chunk_length_bytes.as_slice().try_into().unwrap());
             let chunk : Vec<_> = copy_value.by_ref().take(len_chunk_usize).collect();

             let found_chunk: Vec<u8> = chunk_length_bytes
                .iter()
                .chain(chunk.iter())
                .copied()
                .collect();

             let ch_R  = Chunk::try_from(&found_chunk[..]);
             match ch_R {
                 Ok(c) => chunks.push(c),
                 Err(_) => return Err("Invalid Chunk while creating PNG")
             }
         }

         Ok(Self {
            chunks: chunks,
            header: Png::STANDARD_HEADER
         })
    }
}

impl Png {
    pub const STANDARD_HEADER : [u8;8] = [137,80,78,71,13,10,26,10];
    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Self {
            chunks: chunks,
            header: Png::STANDARD_HEADER,
        }
    }
    pub fn append_chunk(&mut self, chunk: Chunk)  {
        self.chunks.push(chunk);
    }

    pub fn remove_chunk(&mut self,chunk_type:&str) -> Result<Chunk,&'static str> {
        // can use retain instead of finding position and matching it
        let position = self.chunks.iter().position(|i| {
            i.chunk_type.to_string() == chunk_type
        });
        match position {
            Some(pos) => Ok(self.chunks.remove(pos)),
            None => Err("couldn't find the chunktype") 
        }

    }
    pub fn header(&self) -> &[u8; 8] {
        &self.header

    }
    pub fn chunks(&self) -> &[Chunk] {
        self.chunks.as_ref()
    }

    pub fn chunk_by_type(&self, chunk_type:&str) -> Option<&Chunk> {
        self.chunks.iter().find(|&i|{
            i.chunk_type.to_string() == chunk_type
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let data = self.chunks.iter()
        .map(
            |chunk| {
                chunk.as_bytes()
            }
        )
        .reduce(
            |acc,n| {
                acc.iter()
                .chain( n.iter())
                .copied()
                .collect()
            }
        );
        data.unwrap()
    }
}