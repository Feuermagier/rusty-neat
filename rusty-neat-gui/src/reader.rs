use std::{fs, path::Path};
use std::sync::Arc;

use crate::model::{generation::Generation, neat::NeatModel};
use im::Vector;
use rusty_neat_interchange::{
    generation,
    neat_result::{self, PrintableNeatResult},
};

pub fn read(path: &Path) -> Result<NeatModel, String> {
    let mut generations = read_all_generations(path)?;
    let result = read_result(path)?;
    generations.sort_by(|x, y| x.generation.cmp(&y.generation));

    Ok(NeatModel {
        generations: generations,
        current_generation: None,
        current_species: None,
        current_genome: None,
        result: result.map(|r| (&r).into()),
    })
}

fn read_all_generations(path: &Path) -> Result<Vector<Arc<Generation>>, String> {
    let mut generations = Vector::new();

    for entry in fs::read_dir(path).map_err(|err| err.to_string())? {
        let entry = entry.map_err(|err| err.to_string())?.path();
        if entry.is_file() && entry.file_name().unwrap().to_str().unwrap().contains("gen") {
            generations.push_back(Arc::new(generation::read(&entry)?));
        }
    }

    Ok(generations)
}

fn read_result(path: &Path) -> Result<Option<PrintableNeatResult>, String> {
    for entry in fs::read_dir(path).map_err(|err| err.to_string())? {
        let entry = entry.map_err(|err| err.to_string())?.path();
        if entry.is_file()
            && entry
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .contains("result")
        {
            return neat_result::read(&entry).map(|r| Some(r));
        }
    }
    Ok(None)
}
