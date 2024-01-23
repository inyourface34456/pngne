mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use chunk_type::ChunkType;
use chunk::{Chunk, Error};
use png::Png;

// pub type Error = Box<dyn std::error::Error>;
// pub type Result<T> = std::result::Result<T, Error>;

fn chunk_from_strings(chunk_type: &str, data: &str) -> Result<Chunk, Error> {
    use std::str::FromStr;

    let chunk_type = ChunkType::from_str(chunk_type)?;
    let data: Vec<u8> = data.bytes().collect();

    Ok(Chunk::new(chunk_type, data))
}

fn testing_chunks() -> Vec<Chunk> {
    vec![
        chunk_from_strings("FrSt", "I am the first chunkd").unwrap(),
        chunk_from_strings("miDl", "I am another chunkd").unwrap(),
        chunk_from_strings("LASt", "I am the last chunkd").unwrap(),
    ]
}

fn main() {
  let png = Png::from_chunks(testing_chunks());

  println!("{}", png)
}
