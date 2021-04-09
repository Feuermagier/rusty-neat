use std::{
    cell::RefCell,
    cmp::{max, min},
    rc::Rc,
};

use serde::{Deserialize, Serialize};

use rand::prelude::SliceRandom;

use crate::{
    config_util::assert_not_negative,
    gene_pool::GenePool,
    genome::{CrossoverConfig, EvaluationConfig, MutationConfig},
    organism::Organism,
    population::Population,
    species::Species,
};

pub(crate) fn reproduce(
    population: &mut Population,
    pool: Rc<RefCell<GenePool>>,
    config: Rc<ReproductionConfig>,
    evaluation_config: Rc<EvaluationConfig>,
) -> Vec<Organism> {
    match config.global_strategy {
        GlobalReproductionStrategy::Probability => {
            reproduce_probabilistic(population, pool, config.as_ref(), evaluation_config)
        }
        GlobalReproductionStrategy::Fair => {
            reproduce_fair(population, pool, config.as_ref(), evaluation_config)
        }
    }
}

fn reproduce_fair(
    population: &mut Population,
    pool: Rc<RefCell<GenePool>>,
    config: &ReproductionConfig,
    evaluation_config: Rc<EvaluationConfig>,
) -> Vec<Organism> {
    let mut new_population: Vec<Organism> = Vec::with_capacity(config.organism_count);

    let mut total_fitness: f64 = population
        .species
        .iter_mut()
        .map(|s| s.adjusted_fitness())
        .sum();
    if total_fitness == 0.0 {
        total_fitness = 1.0;
    }

    for species in &mut population.species {
        let target_count = (species.adjusted_fitness() / total_fitness
            * config.organism_count as f64)
            .round() as usize;
        reproduce_species(
            species,
            target_count,
            Rc::clone(&pool),
            config,
            Rc::clone(&evaluation_config),
            &mut new_population,
        )
    }

    new_population
}

fn reproduce_probabilistic(
    population: &mut Population,
    pool: Rc<RefCell<GenePool>>,
    config: &ReproductionConfig,
    evaluation_config: Rc<EvaluationConfig>,
) -> Vec<Organism> {
    todo!();
}

fn reproduce_species(
    species: &mut Species,
    target_count: usize,
    pool: Rc<RefCell<GenePool>>,
    config: &ReproductionConfig,
    evaluation_config: Rc<EvaluationConfig>,
    new_population: &mut Vec<Organism>,
) {
    // Organismen innerhalb der Spezies sortieren
    species.organisms.sort_unstable();

    // Die schlechtesten Limit organismen werden für die Kreuzung nicht betrachtet
    let limit = max(
        (species.organisms.len() as f64 * config.kill_ratio) as usize,
        species.organisms.len() - 1,
    );

    // In der Spezies müssen nach der Reduktion mindestens min_species_size viele Organismen sein
    let target_count = max(target_count, config.min_species_size);

    // Elitismus
    let elitism_count = min(config.elitism_count, target_count);
    if config.allow_elitism && species.organisms.len() >= config.elitism_limit {
        new_population.extend(
            species
                .organisms
                .iter()
                .skip(species.organisms.len() - config.elitism_count)
                .map(|x| (**x).clone()),
        );
    }

    // Reine Mutationen
    let mutation_count = (config.mutation_ratio * (target_count - elitism_count) as f64) as usize;
    for _ in 0..mutation_count {
        let parent = select_parent(species, &config.species_strategy, limit);
        let mut offspring = (*parent).clone();
        mutate_organism(&mut offspring, &pool, target_count, config);
        offspring.fitness = None;
        new_population.push(offspring);
    }

    // Kreuzung
    for _ in 0..(target_count - elitism_count - mutation_count) {
        let first_parent = select_parent(species, &config.species_strategy, limit);
        let second_parent = select_parent(species, &config.species_strategy, limit);

        let mut offspring = Organism::new(
            first_parent
                .genome
                .crossover(&second_parent.genome, &pool.borrow(), &config.crossover),
            Rc::clone(&pool),
            Rc::clone(&evaluation_config),
        );

        mutate_organism(&mut offspring, &pool, target_count, config);
        new_population.push(offspring);
    }
}

fn select_parent(
    species: &Species,
    strategy: &SpeciesReproductionStrategy,
    limit: usize,
) -> Rc<Organism> {
    match strategy {
        SpeciesReproductionStrategy::Random => Rc::clone(
            species.organisms[limit..]
                .choose(&mut rand::thread_rng())
                .unwrap(),
        ),
        //SpeciesReproductionStrategy::AdjustedRandom => Rc::clone(species.organisms.iter().choose_weighted(&mut rand::thread_rng(), |o| o.fitness.unwrap()).unwrap())
    }
}

fn mutate_organism(
    organism: &mut Organism,
    pool: &Rc<RefCell<GenePool>>,
    species_size: usize,
    config: &ReproductionConfig,
) {
    if species_size >= config.large_species_size {
        organism
            .genome
            .mutate(&mut pool.borrow_mut(), &config.large_intensity_config);
    } else {
        organism
            .genome
            .mutate(&mut pool.borrow_mut(), &config.large_intensity_config);
    }
}

#[derive(Serialize, Deserialize)]
pub struct ReproductionConfig {
    pub organism_count: usize,   // Anzahl der Organismen in jeder Generation
    pub min_species_size: usize, // Minimale Anzahl an Organismen in einer Spezies
    pub kill_ratio: f64, // Anteil der schlechtesten Organismen, die für die Reproduktion nicht betrachtet werden
    pub mutation_ratio: f64, // Anteil der neuen Organismen, die nur mittels Mutation erzeugt werden (abzüglich des Elitismus)
    pub allow_elitism: bool, // Ob Genome unverändert übernommen werden dürfen
    pub elitism_limit: usize, // Minimale Anzahl an Genomen in einer Spezies, damit elitism_count Genome unverändert in die nächste Generation übernommen werden
    pub elitism_count: usize, // Anzahl der Organismen einer Spezies die unverändert in die nächste Generation übernommen werden. Muss <= elitism_limit sein
    pub global_strategy: GlobalReproductionStrategy, // Wie die Organismen auf die Spezies verteilt werden sollen
    pub species_strategy: SpeciesReproductionStrategy, // Wie die Organismen sich innerhalb einer Spezies reproduzieren
    pub large_species_size: usize, // Ab dieser Anzahl an Organismen zählt eine Spezies als groß und mutiert stärker
    pub crossover: CrossoverConfig, // Wie die Kreuzung funktionieren soll
    pub small_intensity_config: MutationConfig, // Wie eine Mutation mit geringer Intensität erfolgen soll
    pub large_intensity_config: MutationConfig, // Wie eine Mutation mit hoher Intensität erfolgen soll
}

impl ReproductionConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.organism_count == 0 {
            return Err(String::from("organism_count must not be 0"));
        }
        assert_not_negative(self.kill_ratio, "kill_ratio")
            .and(assert_not_negative(self.mutation_ratio, "mutation_ratio"))
            .and(self.small_intensity_config.validate())
            .and(self.crossover.validate())
            .and(self.large_intensity_config.validate())
    }
}

#[derive(Serialize, Deserialize)]
pub enum GlobalReproductionStrategy {
    Probability, // Bessere Spezies haben eine höhere Wahrscheinlichkeit, neue Organismen stellen zu dürfen
    Fair, // Jede Spezies bekommt eine feste Anzahl an neuen Organismen zugeteilt, die von ihrer Fitness abhängt
}

#[derive(Serialize, Deserialize)]
pub enum SpeciesReproductionStrategy {
    Random, // Es werden zufällige Eltern ausgewählt
            // TODO AdjustedRandom    // Bessere Organismen haben eine höhere Wahrscheinlichkeit, Eltern zu werden
}
