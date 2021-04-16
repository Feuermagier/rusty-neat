use std::{error::Error, fs, path::Path};

use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone, Copy)]
pub enum FileType {
    PrettyJSON,
    CompactJSON,
    Bincode,
}

impl FileType {
    pub fn to_ext(&self) -> &str {
        match self {
            FileType::PrettyJSON => ".json",
            FileType::CompactJSON => ".cjson",
            FileType::Bincode => ".bin"
        }
    }

    pub fn from_ext(ext: &str) -> Result<Self, &str> {
        match ext {
            "json" => Ok(FileType::PrettyJSON),
            "cjson" => Ok(FileType::CompactJSON),
            "bin" => Ok(FileType::Bincode),
            _ => Err("Unknown file extension")
        }
    }
}

pub(crate) fn write<T: Serialize>(
    path: &Path,
    content: T,
    file_type: FileType,
) -> Result<(), String> {
    let file_content = match file_type {
        FileType::PrettyJSON => serde_json::to_string_pretty(&content)
            .map_err(|e| stringify(e, path))?
            .as_bytes()
            .to_vec(),
        FileType::CompactJSON => serde_json::to_string(&content)
            .map_err(|e| stringify(e, path))?
            .as_bytes()
            .to_vec(),
        FileType::Bincode => bincode::serialize(&content).map_err(|e| stringify(e, path))?,
    };
    Ok(fs::write(path, file_content).map_err(|e| stringify(e, path))?)
}

pub(crate) fn read<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    if path.is_dir() {
        return Err("path is an directory".to_string());
    }

    let file_content = fs::read(path).map_err(|e| stringify(e, path))?;
    match FileType::from_ext(path.extension().expect("file has no extension").to_str().expect("Invalid extension: Could not be converted to a rust string"))? {
        FileType::PrettyJSON
        | FileType::CompactJSON => serde_json::from_slice(&file_content).map_err(|e| stringify(e, path)),
        FileType::Bincode => bincode::deserialize(&file_content).map_err(|e| stringify(e, path))
    }
}

fn stringify<E: Error>(err: E, file: &Path) -> String {
    format!("Error while reading file '{}': {}", file.to_str().unwrap_or("<path not printable>"), err.to_string())
}
