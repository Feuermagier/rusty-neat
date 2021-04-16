use std::sync::Arc;

use druid::{Data, Lens};
use rusty_neat_interchange::neat_result::PrintableNeatResult;

use super::genome::Genome;
#[derive(Clone, Data, Lens)]
pub struct NeatResult {
    pub best_genome: Arc<Genome>,
    pub best_fitness: f64
}

impl From<&PrintableNeatResult> for NeatResult {
    fn from(printable: &PrintableNeatResult) -> Self {
        Self {
            best_genome: Arc::new((&printable.best_genome, &printable.final_pool).into()),
            best_fitness: printable.best_fitness,
        }
    }
}