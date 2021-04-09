use std::{error::Error, fs};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub enum FileType {
  PrettyJSON,
  CompactJSON,
  Bincode
}

pub(crate) fn write<T: Serialize>(path: &str, content: T, file_type: FileType) -> Result<(), String> {
  let file_content = match file_type {
      FileType::PrettyJSON => serde_json::to_string_pretty(&content).map_err(stringify)?.as_bytes().to_vec(),
      FileType::CompactJSON => serde_json::to_string(&content).map_err(stringify)?.as_bytes().to_vec(),
      FileType::Bincode => bincode::serialize(&content).map_err(stringify)?
  };
  Ok(fs::write(path, file_content).map_err(stringify)?)
}

pub(crate) fn read<T: DeserializeOwned>(path: &str, file_type: FileType) -> Result<T, String> {
  let file_content = fs::read(path).map_err(stringify)?;
  match file_type {
      FileType::PrettyJSON => serde_json::from_slice(&file_content).map_err(stringify),
      FileType::CompactJSON => serde_json::from_slice(&file_content).map_err(stringify),
      FileType::Bincode => bincode::deserialize(&file_content).map_err(stringify),
  }
}

fn stringify<E: Error>(err: E) -> String {
  err.to_string()
}