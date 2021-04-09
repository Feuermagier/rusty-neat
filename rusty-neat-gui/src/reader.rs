use std::{fs, path::Path};

use rusty_neat_interchange::{generation::{self, PrintableGeneration}, neat_result::{self, PrintableNeatResult}};

pub fn read_all_generations(path: &str) -> Result<Vec<PrintableGeneration>, String> {
  let mut generations = Vec::new();

  for entry in fs::read_dir(path).map_err(|err| err.to_string())? {
    let entry = entry.map_err(|err| err.to_string())?.path();
    if entry.is_file() && entry.file_name().unwrap().to_str().unwrap().contains("gen") {
      generations.push(generation::read(&entry)?);
    }
  };

  Ok(generations)
}

pub fn read_result(path: &str) -> Result<PrintableNeatResult, String> {
  for entry in fs::read_dir(path).map_err(|err| err.to_string())? {
    let entry = entry.map_err(|err| err.to_string())?.path();
    if entry.is_file() && entry.file_name().unwrap().to_str().unwrap().contains("result") {
      return neat_result::read(&entry);
    }
  };
  Err("Found no result file".to_owned())
}