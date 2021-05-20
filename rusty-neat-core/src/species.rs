use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, RwLock},
};

use rand::prelude::SliceRandom;
use rusty_neat_interchange::species::PrintableSpecies;

use crate::{
    config_util,
    gene_pool::GenePool,
    genome::{DistanceConfig, EvaluationConfig},
    organism::Organism,
};

use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub(crate) struct Species {
    pub(crate) organisms: Vec<Arc<Organism>>,
    pub(crate) id: usize,
    representative: Arc<Organism>,
    fitness: Option<f64>,
    config: Arc<SpeciesConfig>,
}

impl Species {
    pub fn new(representative: Arc<Organism>, config: Arc<SpeciesConfig>, id: usize) -> Species {
        Species {
            organisms: Vec::new(),
            representative,
            fitness: Option::None,
            config,
            id,
        }
    }

    /*
    pub fn from_printable(
        printable: &PrintableSpecies,
        pool: Arc<RwLock<GenePool>>,
        config: Arc<SpeciesConfig>,
        evaluation_config: Arc<EvaluationConfig>,
    ) -> Self {
        let mut species = Species::new(
            Arc::from(Organism::from_printable(
                &printable.representative,
                Arc::clone(&pool),
                Arc::clone(&evaluation_config),
            )),
            Arc::clone(&pool),
            Arc::clone(&config),
            printable.id
        );

        species.fitness = printable.fitness;

        for organism in &printable.organisms {
            species.organisms.push(Arc::from(Organism::from_printable(
                organism,
                Arc::clone(&pool),
                Arc::clone(&evaluation_config),
            )));
        }

        species
    }
    */

    pub fn adjusted_fitness(&mut self) -> f64 {
        if self.fitness.is_none() {
            self.fitness = self.try_adjusted_fitness();
        }

        self.fitness.unwrap()
    }

    pub fn add_organism(&mut self, organism: Arc<Organism>) {
        self.organisms.push(organism);
        self.fitness = None;
    }

    pub fn matches(&self, organism: Arc<Organism>, config: Arc<DistanceConfig>) -> bool {
        self.representative.distance(&organism, config) <= self.config.species_distance_tolerance
    }

    pub fn select_new_representative(&self) -> Arc<Organism> {
        match self.config.representative {
            ReprentativeSelection::First => Arc::clone(self.organisms.iter().next().unwrap()),
            ReprentativeSelection::Random => {
                Arc::clone(self.organisms.choose(&mut rand::thread_rng()).unwrap())
            }
        }
    }

    fn try_adjusted_fitness(&self) -> Option<f64> {
        if self.organisms.is_empty() {
            None
        } else {
            Some(
                match self.config.fitness {
                    FitnessStrategy::Best => self
                        .organisms
                        .iter()
                        .map(|o| o.fitness.unwrap())
                        .max_by(|x, y| x.partial_cmp(y).unwrap())
                        .unwrap(),
                    FitnessStrategy::Mean => {
                        self.organisms
                            .iter()
                            .map(|o| o.fitness.unwrap())
                            .sum::<f64>()
                            / self.organisms.len() as f64
                    }
                } / self.organisms.len() as f64,
            ) // Die Fitness wird durch die Anzahl der Organismen in der Spezies geteilt (Explicit Fitness Sharing)
        }
    }
}

impl Into<PrintableSpecies> for &Species {
    fn into(self) -> PrintableSpecies {
        let mut printable = PrintableSpecies {
            representative: self.representative.as_ref().into(),
            organisms: Vec::with_capacity(self.organisms.len()),
            fitness: self.try_adjusted_fitness(),
            id: self.id,
        };

        for organism in &self.organisms {
            printable.organisms.push(organism.as_ref().into());
        }

        printable
    }
}

#[derive(Serialize, Deserialize)]
pub struct SpeciesConfig {
    pub representative: ReprentativeSelection, // Wie der Representative einer Spezies ausgewählt werden soll
    pub fitness: FitnessStrategy, // Wie die Fitness einer Spezies berechnet werden soll
    pub species_distance_tolerance: f64, // Maximaler Abstande der Genome inenrhalb eines Species zum Representative
}

impl SpeciesConfig {
    pub fn validate(&self) -> Result<(), String> {
        config_util::assert_not_negative(
            self.species_distance_tolerance,
            "species_distance_tolerance",
        )
    }
}

// Strategie um den Representative einer Spezies auszuwählen
#[derive(Serialize, Deserialize)]
pub enum ReprentativeSelection {
    First, // Erster Organismus (nicht zufällig, aber auch nicht deterministisch)
    Random, // Zufälliger Organismus
           // TODO: CLOSEST     // Organisumus am nächsten zum alten Repräsentanten
}

// Strategie um die Fitness einer Spezies zu berechnen
#[derive(Serialize, Deserialize)]
pub enum FitnessStrategy {
    Mean, // Mittelwert aller Organismen in der Spezies
    Best, // Bester Organismus in der Spezies
}
