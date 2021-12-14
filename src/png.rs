struct Png {}

impl Png {
    fn from_chunks(chunks: Vec<Chunk>) -> Png {

    }
    fn append_chunk(&mut self, chunk: Chunk)  {

    }

    fn remove_chunk(&mut self,chunk_type:&str) -> Result<Chunk> {

    }
    fn header(&self) -> &[u8; 8] {

    }
    fn chunks(&self) -> &[Chunk] {

    }

    fn chunk_by_type(&self, chunk_type:&str) -> Option<&Chunk> {

    }

    fn as_bytes(&self) -> Vec<u8> {

    }
}