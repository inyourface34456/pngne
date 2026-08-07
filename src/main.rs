#![allow(unused)]
mod chunk;
mod chunk_type;
mod error;
mod png;

use std::io::{Read, Write};
use std::str::FromStr;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::error::Error as Errors;
use crate::png::Png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

enum Commands {
    Encode{
        file_path: String,
        chunk_type: String,
        message: String,
        output_file: String,
    },
    Decode {
        file_path: String,
        chunk_type: String,
    },
    Remove {
        file_path: String,
        chunk_type: String,
    },
    Print(String),
    Exit(Errors),
}

impl Commands {
    #[cfg(target_os = "windows")]
    const WINDOWS_TOMFOOLERY: usize = 1;
    #[cfg(not(target_os = "windows"))]
    const WINDOWS_TOMFOOLERY: usize = 0;
    fn new(args: &[String]) -> Self {
        match args[1-Self::WINDOWS_TOMFOOLERY].to_lowercase().as_str() {
            "encode" => {
                let file_path: String =  match args.get(2-Self::WINDOWS_TOMFOOLERY) {
                    Some(n) => n.clone(),
                    None => return Self::Exit(Errors::IncorrectArgs)
                };
                let chunk_type: String = match args.get(3-Self::WINDOWS_TOMFOOLERY) {
                    Some(n) => n.clone(),
                    None => return Self::Exit(Errors::IncorrectArgs)
                };
                let message: String = match args.get(4-Self::WINDOWS_TOMFOOLERY) {
                    Some(n) => n.clone(),
                    None => return Self::Exit(Errors::IncorrectArgs)
                };
                let output_file: String = args.get(5-Self::WINDOWS_TOMFOOLERY).unwrap_or(&file_path).clone();
                Self::Encode { file_path, chunk_type, message, output_file }
            }
            "decode" => {
                let file_path: String =  match args.get(2-Self::WINDOWS_TOMFOOLERY) {
                    Some(n) => n.clone(),
                    None => return Self::Exit(Errors::IncorrectArgs)
                };
                let chunk_type: String = match args.get(3-Self::WINDOWS_TOMFOOLERY) {
                    Some(n) => n.clone(),
                    None => return Self::Exit(Errors::IncorrectArgs)
                };
                Self::Decode { file_path, chunk_type }
            }
            "remove" => {
                let file_path: String =  match args.get(2-Self::WINDOWS_TOMFOOLERY) {
                    Some(n) => n.clone(),
                    None => return Self::Exit(Errors::IncorrectArgs)
                };
                let chunk_type: String = match args.get(3-Self::WINDOWS_TOMFOOLERY) {
                    Some(n) => n.clone(),
                    None => return Self::Exit(Errors::IncorrectArgs)
                };
                Self::Remove { file_path, chunk_type }
            }
            "print" => {
                let file_path: String =  match args.get(2-Self::WINDOWS_TOMFOOLERY) {
                    Some(n) => n.clone(),
                    None => return Self::Exit(Errors::IncorrectArgs)
                };
                Self::Print(file_path)
            }
            _ => Self::Exit(Errors::IncorrectArgs)
        }
    }

    fn execute(&self) -> Result<()> {
        match self {
            Self::Decode { file_path, chunk_type } => {
                let mut file_data = vec![];
                let mut file = std::fs::File::open(file_path).map_err(|f| Errors::FileDoesNotExist(f.to_string()))?;
                _ = file.read_to_end(&mut file_data);
                let png = Png::try_from(file_data.as_slice())?;
                let chunk = png.chunk_by_type(chunk_type).ok_or(Errors::ChunkTypeNotFound)?;
                println!("{chunk}");
                Ok(())
            }
            Self::Encode { file_path, chunk_type, message, output_file } => {
                let mut file_data = vec![];
                let mut file = std::fs::File::open(file_path).map_err(|f| Errors::FileDoesNotExist(f.to_string()))?;
                _ = file.read_to_end(&mut file_data);
                let mut png = Png::try_from(file_data.as_slice())?;
                png.append_chunk(Chunk::new(ChunkType::from_str(chunk_type)?, message.as_bytes().to_vec()));
                std::fs::File::create(output_file)?.write_all(png.as_bytes().as_slice())?;
                Ok(())
            }
            Self::Remove { file_path, chunk_type } => {
                let mut file_data = vec![];
                let mut file = std::fs::File::open(file_path).map_err(|f| Errors::FileDoesNotExist(f.to_string()))?;
                _ = file.read_to_end(&mut file_data);
                let mut png = Png::try_from(file_data.as_slice())?;
                png.remove_first_chunk(chunk_type)?;
                std::fs::File::create(file_path)?.write_all(png.as_bytes().as_slice())?;
                Ok(())
            }
            Self::Print(file) => {
                let mut file_data = vec![];
                let mut file = std::fs::File::open(file).map_err(|f| Errors::FileDoesNotExist(f.to_string()))?;
                _ = file.read_to_end(&mut file_data);
                let png = Png::try_from(file_data.as_slice())?;
                println!("{png}");
                Ok(())
            }
            Self::Exit(err) => {
                match err {
                    Errors::IncorrectArgs => {
                        println!(r"Usage of pngme:
encode [file_name] [chunk_type] [message] [output_file?]: encodes the message within the specified filename and chunk type
decode [file_name] [chunk_type]: prints the first chunk of the spcified type (note: if the data is not utf8 decodeable, it will only print the length of the data)
remove [file_name] [chunk_type]: removes the first chunk of the specified type
print [file_name]: prints all chunks in the png file");
                        Ok(())
                    }
                    anything_else => Err(<error::Error as std::convert::Into<Error>>::into(anything_else.clone()))
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let args = Commands::new(std::env::args().collect::<Vec<_>>().as_slice());
    args.execute()
}
