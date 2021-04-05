use std::{cell::RefCell, rc::Rc};

use rand::prelude::{SliceRandom};

use crate::{config_util, gene_pool::GenePool, genome::DistanceConfig, organism::Organism};

#[derive(Clone)]
pub(crate) struct Species {
  pub(crate) organisms: Vec<Rc<Organism>>,
  representative: Rc<Organism>,
  fitness: Option<f64>,
  pool: Rc<RefCell<GenePool>>,
  config: Rc<SpeciesConfig>
}

impl Species {
  pub fn new(representative: Rc<Organism>, pool: Rc<RefCell<GenePool>>, config: Rc<SpeciesConfig>) -> Species {
    Species {
      organisms: Vec::new(),
      representative,
      fitness: Option::None,
      pool,
      config
    }
  }

  pub fn adjusted_fitness(&mut self) -> f64 {
    if self.fitness.is_none() {
      self.fitness = Option::Some(match self.config.fitness {
        FitnessStrategy::BEST => self.organisms.iter().map(|o| o.fitness.unwrap()).max_by(|x, y| x.partial_cmp(y).unwrap()).unwrap(),
        FitnessStrategy::MEAN => self.organisms.iter().map(|o| o.fitness.unwrap()).sum::<f64>() / self.organisms.len() as f64
      } / self.organisms.len() as f64); // Die Fitness wird durch die Anzahl der Organismen in der Spezies geteilt (Explicit Fitness Sharing)
    }

    self.fitness.unwrap()
  }

  pub fn add_organism(&mut self, organism: Rc<Organism>) {
    self.organisms.push(organism);
    self.fitness = None;
  }

  pub fn matches(&self, organism: Rc<Organism>, config: Rc<DistanceConfig>) -> bool {
    self.representative.distance(&organism, config) <= self.config.species_distance_tolerance
  }

  pub fn select_new_representative(&self) -> Rc<Organism> {
    match self.config.representative {
      ReprentativeSelection::FIRST => Rc::clone(self.organisms.iter().next().unwrap()),
      ReprentativeSelection::RANDOM => Rc::clone(self.organisms.choose(&mut rand::thread_rng()).unwrap())
    }
  }
}

pub struct SpeciesConfig {
  pub representative: ReprentativeSelection,  // Wie der Representative einer Spezies ausgewählt werden soll
  pub fitness: FitnessStrategy,               // Wie die Fitness einer Spezies berechnet werden soll
  pub species_distance_tolerance: f64,        // Maximaler Abstande der Genome inenrhalb eines Species zum Representative
}

impl SpeciesConfig {
  pub fn validate(&self) -> Result<(), String> {
    config_util::assert_not_negative(self.species_distance_tolerance, "species_distance_tolerance")
  }
}

// Strategie um den Representative einer Spezies auszuwählen
pub enum ReprentativeSelection {
  FIRST,      // Erster Organismus (nicht zufällig, aber auch nicht deterministisch)
  RANDOM,     // Zufälliger Organismus
  // TODO: CLOSEST     // Organisumus am nächsten zum alten Repräsentanten
}

// Strategie um die Fitness einer Spezies zu berechnen
pub enum FitnessStrategy {
  MEAN,     // Mittelwert aller Organismen in der Spezies
  BEST      // Bester Organismus in der Spezies
}