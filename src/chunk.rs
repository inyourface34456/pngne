use crc::crc32::checksum_ieee;
use crate::chunk_type::ChunkType;
use std::convert::TryFrom;
use std::fmt;

struct Chunk {
  length: u32,
  chunk_type: ChunkType,
  data: Vec<u8>,
  crc: u32
}

#[derive(Debug)]
enum Error {
  InputTooSmall(usize),
  ChunkTypeNotValid,
  CrcMissMatch(u32, u32),
  NotOk
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl TryFrom<&[u8]> for Chunk {
  type Error = Error;

  fn try_from(value: &[u8]) -> Result<Self, Error> {
    if value.len() < 16 {
      return Err(Error::InputTooSmall(value.len()));
    }
    
    let (data_length, value) = value.split_at(4);
    
    let data_length = match data_length.try_into() {
      Ok(dat) => u32::from_be_bytes(dat) as usize,
      Err(_) => return Err(Error::NotOk)
    };
    
    let (chunk_type, value) = value.split_at(4);

    let chunk_type: &[u8; 4] = match chunk_type.try_into() {
      Ok(dat) => dat,
      Err(_) => return Err(Error::NotOk)
    };
    
    let chunk_type = match ChunkType::try_from(chunk_type) {
      Ok(dat) => dat,
      Err(_) => return Err(Error::ChunkTypeNotValid)
    };

    let (data, value) = value.split_at(data_length);
    let (crc_true, _) = value.split_at(4);

    let bytes: Vec<u8> = chunk_type
      .bytes()
      .iter()
      .chain(data.iter())
      .copied()
      .collect();
    
    let crc = checksum_ieee(&bytes);
    let true_crc = match crc_true.try_into() {
      Ok(dat) => u32::from_be_bytes(dat),
      Err(_) => return Err(Error::NotOk)
    };
  
    if crc != true_crc {
      return Err(Error::CrcMissMatch(crc, true_crc))
    }

    let new = Self {
        length: data_length as u32,
        chunk_type,
        data: data.into(),
        crc
    };
  
    Ok(new)
    
  }
}

impl Chunk {
  pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
    let bytes: Vec<u8> = chunk_type
      .bytes()
      .iter()
      .chain(data.iter())
      .copied()
      .collect();

    let crc = checksum_ieee(&bytes);
    
    Self { 
      length: data.len() as u32, 
      chunk_type, 
      data, 
      crc
   }
  }
  
  pub fn length(&self) -> u32 {
    self.length
  }
  
  pub fn chunk_type(&self) -> &ChunkType {
    &self.chunk_type
  }
  
  pub fn data(&self) -> &[u8] {
    self.data.as_slice()
  }
  
  pub fn crc(&self) -> u32 {
    self.crc
  }
  
  pub fn data_as_string(&self) -> Result<String, Error> {
    match String::from_utf8(self.data.clone()) {
      Ok(dat) => return Ok(dat),
      Err(_) => return Err(Error::NotOk)
    }
  }
  
  pub fn as_bytes(&self) -> Vec<u8> {
    self.data.clone()
  }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = b"This is where your secret message will be!";
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = b"This is where your secret message will be!".to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = b"This is where your secret message will be!";
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = b"This is where your secret message will be!";
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = b"This is where your secret message will be!";
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
