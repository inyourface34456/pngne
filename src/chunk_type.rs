use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Eq, PartialEq)]
struct ChunkType {
    chunk_type: Vec<char>,
}

#[derive(PartialEq)]
enum ChunkTypeError {
    ValueNotInRange,
    StrNotCorrctLngth,
    None,
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut is_error = false;
        let mut error = fmt::Result::Ok(());

        for i in &self.chunk_type {
            match write!(f, "{}", i) {
                Ok(_) => {}
                Err(e) => {
                    is_error = true;
                    error = fmt::Result::Err(e);
                    break;
                }
            }
        }

        if is_error {
            return error;
        } else {
            return fmt::Result::Ok(());
        }
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let mut chunk_type = vec![];
        let mut is_error = false;

        for i in value {
            match i {
                97..=122 => chunk_type.push(i as char),
                _ => is_error = true,
            }
        }

        if is_error {
            Err(ChunkTypeError::ValueNotInRange)
        } else {
            Ok(Self { chunk_type })
        }
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut is_error = ChunkTypeError::StrNotCorrctLngth;
        let mut chunk_type = vec![];

        if s.len() == 4 {
            is_error = ChunkTypeError::None;
        } else {
            return Err(is_error);
        }

        for i in s.chars() {
            chunk_type.push(i)
        }

        if is_error != ChunkTypeError::None {
            Err(is_error)
        } else {
            Ok(Self { chunk_type })
        }
    }
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.chunk_type
            .iter()
            .map(|x| *x as u8)
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap()
    }

    fn is_valid(&self) -> bool {
        true
    }

    fn is_critical(&self) -> bool {
        true
    }

    fn is_public(&self) -> bool {
        true
    }

    fn is_reserved_bit_valid(&self) -> bool {
        true
    }

    fn is_safe_to_copy(&self) -> bool {
        true
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
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
