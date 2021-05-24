use core::f64;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::Path,
    sync::Arc,
};

use rusty_neat_interchange::{
    generation::{self, PrintableGeneration},
    io::FileType,
    neat_result::{self, PrintableNeatResult},
};

use crate::{
    config_util::assert_not_negative,
    gene_pool::GenePool,
    genome::{DistanceConfig, EvaluationConfig, GenomeIdGenerator, NewConnectionWeight},
    organism::Organism,
    reproduction::{self, ReproductionConfig},
    species::{Species, SpeciesConfig},
};

pub struct Population {
    config: Arc<PopulationConfig>,
    pub(crate) pool: GenePool,
    pub(crate) organisms: Vec<Arc<Organism>>,
    pub(crate) species: Vec<Species>,
    next_species_id: usize,
    pub(crate) genome_id_generator: GenomeIdGenerator,
}

impl Population {
    pub fn new(pool: GenePool, config_path: &Path) -> Result<Population, String> {
        let config: PopulationConfig =
            serde_json::from_str(&fs::read_to_string(config_path).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        if let Err(msg) = config.validate() {
            return Err(msg);
        }
        let population = Population {
            pool: pool,
            organisms: Vec::with_capacity(config.reproduction.organism_count),
            config: Arc::from(config),
            species: Vec::with_capacity(1),
            next_species_id: 0,
            genome_id_generator: GenomeIdGenerator::new(),
        };
        Ok(population)
    }

    pub fn evolve<F: Fn(&mut [Organism]) -> ()>(
        &mut self,
        fitness_function: F,
        target_path: &Path,
    ) -> Result<Organism, String> {
        prepare_target_directory(target_path)?;

        self.generate_initial_population(&fitness_function);

        let target_fitness = self.config.target_fitness;
        let mut best_organism = Arc::clone(&self.organisms[0]);

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
                Arc::clone(&self.config.reproduction),
                Arc::clone(&self.config.evaluation),
                generation,
            );

            // Der Benutzer bewertet die Organismen
            println!("Evaluating...");
            fitness_function(&mut new_organisms);

            // TODO: Cloning vermeiden
            println!("Copying organisms...");
            self.organisms = new_organisms.iter().map(|x| Arc::from(x.clone())).collect();

            // Beste erreichte Fitness für die Abbruchbedingung ermitteln
            best_organism = Arc::clone(
                self.organisms
                    .iter()
                    .max_by(|x, y| x.fitness.unwrap().partial_cmp(&y.fitness.unwrap()).unwrap())
                    .unwrap(),
            );
            println!(
                "=> Best genome {}, fitness {:.10} using {} nodes and {} connections ({} enabled)",
                best_organism.genome.id(),
                best_organism.fitness.unwrap(),
                best_organism.genome.node_count(),
                best_organism.genome.connection_count(),
                best_organism.genome.enabled_connection_count()
            );
            println!(
                "=> Gene Pool: {} nodes, {} connections",
                self.pool.nodes.len(),
                self.pool.connections.len()
            );

            // Die Organismen in Spezies einteilen
            println!("Speciating...\n");
            self.speciate();

            self.write_generation(generation, target_path, FileType::Bincode)?;

            generation += 1;
        }

        write_result(
            Arc::clone(&best_organism),
            &self.pool,
            target_path,
            FileType::Bincode,
        );

        Ok((*best_organism).clone())
    }

    fn generate_initial_population<F: Fn(&mut [Organism]) -> ()>(&mut self, fitness_function: F) {
        let mut organisms: Vec<Organism> =
            Vec::with_capacity(self.config.reproduction.organism_count);

        for _ in 0..self.config.reproduction.organism_count {
            organisms.push(Organism::new(
                self.pool.new_genome(
                    &self.config.initial_organism_weight,
                    self.genome_id_generator.next_id(),
                    0,
                ),
                Arc::clone(&self.config.evaluation),
            ));
        }

        fitness_function(&mut organisms);
        self.organisms = organisms.iter().map(|x| Arc::new(x.clone())).collect();

        self.speciate();
    }

    fn speciate(&mut self) {
        let mut new_species: Vec<Species> = Vec::new();

        // Alte Spezien übernehmen
        for species in &self.species {
            new_species.push(Species::new(
                species.select_new_representative(),
                Arc::clone(&self.config.species),
                species.id,
            ));
        }

        // Neue Organismen einteilen
        for organism in &self.organisms {
            let mut found_species = false;
            for species in &mut new_species {
                if species.matches(Arc::clone(organism), Arc::clone(&self.config.distance)) {
                    species.add_organism(Arc::clone(organism));
                    found_species = true;
                    break;
                }
            }
            if !found_species {
                println!("Creating a new species");
                let mut species = Species::new(
                    Arc::clone(&organism),
                    Arc::clone(&self.config.species),
                    self.next_species_id,
                );
                self.next_species_id += 1;
                species.add_organism(Arc::clone(&organism));
                new_species.push(species);
            }
        }

        // Ausgelöschte Spezies entfernen
        new_species.retain(|s| !s.organisms.is_empty());

        self.species = new_species;
    }

    fn write_generation(
        &self,
        generation_number: u32,
        path: &Path,
        file_type: FileType,
    ) -> Result<(), String> {
        let generation = PrintableGeneration {
            generation: generation_number,
            species: self.species.iter().map(|s| s.into()).collect(),
            pool: (&self.pool).into(),
        };

        generation::write(
            generation,
            &path.join("gen-".to_owned() + &generation_number.to_string() + file_type.to_ext()),
            file_type,
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct PopulationConfig {
    pub target_fitness: f64, // Wird diese Fitness erreicht oder überschritten wird abgebrochen
    pub max_generations: u32, // So viele Generationen werden höchstens durchlaufen (0 entspricht unbegrenzt)
    pub initial_organism_weight: NewConnectionWeight, // So wird das Gewicht der Connections in den initialen Genomen bestimmt
    pub distance: Arc<DistanceConfig>,
    pub species: Arc<SpeciesConfig>,
    pub evaluation: Arc<EvaluationConfig>,
    pub reproduction: Arc<ReproductionConfig>,
}

impl PopulationConfig {
    pub fn validate(&self) -> Result<(), String> {
        assert_not_negative(self.target_fitness, "target_fitness")
            .and(self.distance.validate())
            .and(self.species.validate())
            .and(self.reproduction.validate())
    }
}

fn prepare_target_directory(target_path: &Path) -> Result<(), String> {
    if target_path.is_file() {
        return Err("target_path refers to a file".to_owned());
    }

    if target_path.exists() {
        fs::remove_dir_all(target_path).map_err(|err| err.to_string())?;
    }
    fs::create_dir_all(target_path).map_err(|err| err.to_string())
}

fn write_result(
    best_organism: Arc<Organism>,
    final_pool: &GenePool,
    path: &Path,
    file_type: FileType,
) {
    let result = PrintableNeatResult {
        best_genome: (&best_organism.genome).into(),
        best_fitness: best_organism.fitness.unwrap(),
        final_pool: final_pool.into(),
    };

    neat_result::write(
        result,
        &path.join("result".to_owned() + file_type.to_ext()),
        file_type,
    )
    .unwrap();
}
