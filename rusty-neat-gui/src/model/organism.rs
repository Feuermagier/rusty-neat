use std::sync::Arc;

use druid::{Data, Lens};
use rusty_neat_interchange::{gene_pool::PrintableGenePool, organism::PrintableOrganism};

use super::genome::Genome;

#[derive(Clone, Lens, Data)]
pub struct Organism {
    pub genome: Arc<Genome>,
    pub fitness: Option<f64>,
}

impl From<(&PrintableOrganism, &PrintableGenePool)> for Organism {
    fn from((organism, pool): (&PrintableOrganism, &PrintableGenePool)) -> Self {
        Self {
            genome: Arc::new((&organism.genome, pool).into()),
            fitness: organism.fitness.clone(),
        }
    }
}

impl PartialEq for Organism {
    fn eq(&self, other: &Self) -> bool {
        if self.fitness.is_some() && other.fitness.is_some() {
            self.fitness.unwrap().eq(&other.fitness.unwrap())
        } else {
            false
        }
    }
}

impl Eq for Organism {}

impl PartialOrd for Organism {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.fitness.is_some() && other.fitness.is_some() {
            self.fitness.unwrap().partial_cmp(&other.fitness.unwrap())
        } else {
            None
        }
    }
}

impl Ord for Organism {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fitness
            .unwrap()
            .partial_cmp(&other.fitness.unwrap())
            .unwrap()
    }
}