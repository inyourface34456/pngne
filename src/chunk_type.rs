#![allow(clippy::trivially_copy_pass_by_ref)]
use crate::error::Error;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ChunkType {
    png_type: [u8; 4],
}

impl ChunkType {
    // byte mask, they normly dont get seperators
    #[allow(clippy::unreadable_literal)]
    const BYTE_5_MASK: u8 = 0b00100000;

    pub fn bytes(&self) -> [u8; 4] {
        self.png_type
    }

    pub fn is_valid(&self) -> bool {
        self.png_type[2] & Self::BYTE_5_MASK == 0
    }

    pub fn is_critical(&self) -> bool {
        self.png_type[0] & Self::BYTE_5_MASK == 0
    }

    pub fn is_public(&self) -> bool {
        self.png_type[1] & Self::BYTE_5_MASK == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.is_valid()
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.png_type[3] & Self::BYTE_5_MASK != 0
    }

    fn is_letters(value: [u8; 4]) -> bool {
        for char in value {
            match char {
                65..=90 | 97..=122 => {}
                _ => return false,
            }
        }
        true
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if Self::is_letters(value) {
            Ok(ChunkType { png_type: value })
        } else {
            Err(Error::ContainsNumbers(u32::from_be_bytes(value)).into())
        }
    }
}

impl FromStr for ChunkType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: Vec<u8> = s.bytes().collect();

        match bytes.len() {
            4 => {
                let bytes = bytes.as_array().unwrap();
                if Self::is_letters(*bytes) {
                    Ok(Self { png_type: *bytes })
                } else {
                    Err(Error::ContainsNumbers(u32::from_be_bytes(*bytes)).into())
                }
            }
            0..4 => Err(Error::StringTooShort.into()),
            5..usize::MAX => Err(Error::StringTooLong.into()),
            _ => Err(Error::StringNotRightLen.into()),
        }
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}",
            self.png_type[0] as char,
            self.png_type[1] as char,
            self.png_type[2] as char,
            self.png_type[3] as char
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{chunk_type_1}");
        #[allow(clippy::no_effect_underscore_binding)]
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
