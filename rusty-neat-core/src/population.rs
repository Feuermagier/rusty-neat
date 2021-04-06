use core::f64;
use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use crate::{
    config_util::assert_not_negative,
    gene_pool::GenePool,
    genome::{DistanceConfig, EvaluationConfig, NewConnectionWeight},
    organism::Organism,
    reproduction::{self, ReproductionConfig},
    species::{Species, SpeciesConfig},
};

pub struct Population {
    config: Rc<PopulationConfig>,
    pool: Rc<RefCell<GenePool>>,
    pub(crate) organisms: Vec<Rc<Organism>>,
    pub(crate) species: Vec<Species>,
}

impl Population {
    pub fn new(pool: GenePool, config: PopulationConfig) -> Result<Population, String> {
        if let Err(msg) = config.validate() {
            return Err(msg);
        }
        let population = Population {
            pool: Rc::from(RefCell::from(pool)),
            organisms: Vec::with_capacity(1),
            config: Rc::from(config),
            species: Vec::with_capacity(1),
        };
        Ok(population)
    }

    pub fn evolve<F: Fn(&mut [Organism]) -> ()>(&mut self, fitness_function: F) -> Organism {
        self.generate_initial_population(&fitness_function);

        let target_fitness = self.config.target_fitness;
        let mut best_organism = Rc::clone(&self.organisms[0]);

        let max_generations = self.config.max_generations;
        let mut generation = 1;

        while generation <= max_generations && best_organism.fitness.unwrap() <= target_fitness {
            println!(
                "Generation {}: {} organisms, {} species",
                generation,
                self.organisms.len(),
                self.species.len()
            );
            // Neue Organismen erzeugen (durch Elitismus, Kreuzung und Mutationen)
            println!("Reproducing...");
            let mut new_organisms = reproduction::reproduce(
                self,
                Rc::clone(&self.pool),
                Rc::clone(&self.config.reproduction),
                Rc::clone(&self.config.evaluation),
            );

            // Der Benutzer bewertet die Organismen
            println!("Evaluating...");
            fitness_function(&mut new_organisms);

            // TODO: Cloning vermeiden
            println!("Copying organisms...");
            self.organisms = new_organisms.iter().map(|x| Rc::from(x.clone())).collect();

            // Beste erreichte Fitness für die Abbruchbedingung ermitteln
            best_organism = Rc::clone(
                self.organisms
                    .iter()
                    .max_by(|x, y| x.fitness.unwrap().partial_cmp(&y.fitness.unwrap()).unwrap())
                    .unwrap(),
            );
            println!(
                "=> Best fitness: {:.10} using {} nodes",
                best_organism.fitness.unwrap(),
                best_organism.genome.node_count()
            );

            // Die Organismen in Spezies einteilen
            println!("Speciating...\n");
            self.speciate();

            generation += 1;
        }

        (*best_organism).clone()
    }

    fn generate_initial_population<F: Fn(&mut [Organism]) -> ()>(&mut self, fitness_function: F) {
        let mut organisms: Vec<Organism> =
            Vec::with_capacity(self.config.reproduction.organism_count);

        for _ in 0..self.config.reproduction.organism_count {
            organisms.push(Organism::new(
                self.pool
                    .borrow_mut()
                    .new_genome(&self.config.initial_organism_weight),
                Rc::clone(&self.pool),
                Rc::clone(&self.config.evaluation),
            ));
        }

        fitness_function(&mut organisms);
        self.organisms = organisms.iter().map(|x| Rc::from(x.clone())).collect();

        self.speciate();
    }

    fn speciate(&mut self) {
        let mut new_species: Vec<Species> = Vec::new();

        // Alte Spezien übernehmen
        for species in &self.species {
            new_species.push(Species::new(
                species.select_new_representative(),
                Rc::clone(&self.pool),
                Rc::clone(&self.config.species),
            ));
        }

        // Neue Organismen einteilen
        for organism in &self.organisms {
            let mut found_species = false;
            for species in &mut new_species {
                if species.matches(Rc::clone(organism), Rc::clone(&self.config.distance)) {
                    species.add_organism(Rc::clone(organism));
                    found_species = true;
                    break;
                }
            }
            if !found_species {
                let mut species = Species::new(
                    Rc::clone(&organism),
                    Rc::clone(&self.pool),
                    Rc::clone(&self.config.species),
                );
                species.add_organism(Rc::clone(&organism));
                new_species.push(species);
            }
        }

        // Ausgelöschte SPezies entfernen
        new_species.retain(|s| !s.organisms.is_empty());

        self.species = new_species;
    }
}

pub struct PopulationConfig {
    pub target_fitness: f64, // Wird diese Fitness erreicht oder überschritten wird abgebrochen
    pub max_generations: usize, // So viele Generationen werden höchstens durchlaufen (0 entspricht unbegrenzt)
    pub initial_organism_weight: NewConnectionWeight, // So wird das Gewicht der Connections in den initialen Genomen bestimmt
    pub distance: Rc<DistanceConfig>,
    pub species: Rc<SpeciesConfig>,
    pub evaluation: Rc<EvaluationConfig>,
    pub reproduction: Rc<ReproductionConfig>,
}

impl PopulationConfig {
    pub fn validate(&self) -> Result<(), String> {
        assert_not_negative(self.target_fitness, "target_fitness")
            .and(self.distance.validate())
            .and(self.species.validate())
            .and(self.reproduction.validate())
    }
}
