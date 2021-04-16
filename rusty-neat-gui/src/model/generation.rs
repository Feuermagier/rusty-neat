use std::sync::Arc;

use super::{gene_pool::GenePool, organism::Organism, species::Species};

use druid::{Data, Lens};
use im::Vector;
use rusty_neat_interchange::generation::PrintableGeneration;
#[derive(Clone, Lens, Data)]
pub struct Generation {
    pub generation: u32,
    pub species: Vector<Arc<Species>>,
    pub best_organism: Arc<Organism>,
    pub average_fitness: f64,
    pub pool: GenePool
}

impl From<PrintableGeneration> for Generation {
    fn from(generation: PrintableGeneration) -> Self {
        let species: Vector<Arc<Species>> = generation.species.iter().map(|s| Arc::new((s, &generation.pool).into())).collect();
        let best_organism = Arc::clone(species.iter().map(|s| &s.best_organism).max().unwrap());
        let average_fitness = species.iter().map(|s| s.average_fitness).sum::<f64>() / species.len() as f64;

        Self {
            generation: generation.generation,
            species,
            best_organism,
            average_fitness,
            pool: generation.pool.into()
        }
    }
}