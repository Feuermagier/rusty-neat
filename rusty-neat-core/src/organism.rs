use std::{cell::RefCell, fmt, rc::Rc};

use rusty_neat_interchange::organism::PrintableOrganism;
use serde::{Deserialize, Serialize};

use crate::{
    gene_pool::GenePool,
    genome::{DistanceConfig, EvaluationConfig, Genome},
};

#[derive(Clone)]
pub struct Organism {
    pub(crate) genome: Genome,
    pool: Rc<RefCell<GenePool>>,
    evaluation_config: Rc<EvaluationConfig>,
    pub fitness: Option<f64>,
}

impl Organism {
    pub(crate) fn new(
        genome: Genome,
        pool: Rc<RefCell<GenePool>>,
        evaluation_config: Rc<EvaluationConfig>,
    ) -> Organism {
        Organism {
            genome,
            pool,
            evaluation_config,
            fitness: None,
        }
    }

    pub fn from_printable(
        printable: &PrintableOrganism,
        pool: Rc<RefCell<GenePool>>,
        evaluation_config: Rc<EvaluationConfig>,
    ) -> Self {
        Organism {
            genome: Genome::from_printable(&printable.genome, &pool.borrow()),
            pool: Rc::clone(&pool),
            evaluation_config,
            fitness: printable.fitness,
        }
    }

    pub fn evaluate(&mut self, input: &[f64]) -> Vec<f64> {
        self.genome
            .evaluate(input, &self.pool.borrow(), self.evaluation_config.as_ref())
    }

    pub(crate) fn distance(&self, other: &Organism, config: Rc<DistanceConfig>) -> f64 {
        self.genome.distance(&other.genome, config.as_ref())
    }
}

impl Into<PrintableOrganism> for &Organism {
    fn into(self) -> PrintableOrganism {
        PrintableOrganism {
            genome: (&self.genome).into(),
            fitness: self.fitness,
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

impl fmt::Debug for Organism {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Fitness: {:.10}, Genome: {:?}",
            self.fitness.unwrap_or(-1.0),
            self.genome
        )
    }
}
