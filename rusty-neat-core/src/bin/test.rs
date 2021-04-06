use std::rc::Rc;

use rand_distr::Normal;
use rusty_neat_core::{
    activation::Activation,
    gene_pool::GenePool,
    genome::{
        CrossoverConfig, CrossoverWeightStrategy, DistanceConfig, EvaluationConfig, Genome,
        MutationConfig, NewConnectionWeight,
    },
    population::{Population, PopulationConfig},
    reproduction::{GlobalReproductionStrategy, ReproductionConfig, SpeciesReproductionStrategy},
    serialize,
    species::{FitnessStrategy, ReprentativeSelection, SpeciesConfig},
};

fn main() {
    let config = PopulationConfig {
        target_fitness: 3.8,
        max_generations: 200,
        initial_organism_weight: NewConnectionWeight::Random(Normal::new(0.0, 3.0).unwrap()),
        distance: Rc::from(DistanceConfig {
            c1: 1.0,
            c2: 1.0,
            c3: 0.3,
        }),
        species: Rc::from(SpeciesConfig {
            representative: ReprentativeSelection::RANDOM,
            fitness: FitnessStrategy::MEAN,
            species_distance_tolerance: 3.0,
        }),
        evaluation: Rc::from(EvaluationConfig {
            bias: 0.0,
            activation: Activation::SIGMOID,
        }),
        reproduction: Rc::from(ReproductionConfig {
            organism_count: 1000,
            min_species_size: 10,
            kill_ratio: 0.5,
            mutation_ratio: 0.5,
            allow_elitism: true,
            elitism_limit: 20,
            elitism_count: 2,
            global_strategy: GlobalReproductionStrategy::Fair,
            species_strategy: SpeciesReproductionStrategy::Random,
            large_species_size: 30,
            crossover: CrossoverConfig {
                disable_connection_prob: 0.75,
                weight_strategy: CrossoverWeightStrategy::Random,
            },
            small_intensity_config: MutationConfig {
                change_weight_prob: 0.8,
                random_weight_dist: Normal::new(0.0, 3.0).unwrap(),
                shift_weight_prob: 0.9,
                shift_weight_dist: Normal::new(0.0, 0.5).unwrap(),
                add_node_prob: 0.03,
                add_connection_prob: 0.05,
                add_connection_retry_count: 100,
                new_connection_weight: NewConnectionWeight::Random(Normal::new(0.0, 3.0).unwrap()),
                toggle_connection_prob: 0.08,
            },
            large_intensity_config: MutationConfig {
                change_weight_prob: 0.8,
                random_weight_dist: Normal::new(0.0, 3.0).unwrap(),
                shift_weight_prob: 0.9,
                shift_weight_dist: Normal::new(0.0, 0.8).unwrap(),
                add_node_prob: 0.03,
                add_connection_prob: 0.3,
                add_connection_retry_count: 100,
                new_connection_weight: NewConnectionWeight::Random(Normal::new(0.0, 3.0).unwrap()),
                toggle_connection_prob: 0.00,
            },
        }),
    };

    let pool = GenePool::new_dense(3, 1);
    let mut population = Population::new(pool, config).unwrap();
    let none = vec![0.0, 0.0, 1.0];
    let first = vec![1.0, 0.0, 1.0];
    let second = vec![0.0, 1.0, 1.0];
    let both = vec![1.0, 1.0, 1.0];
    let mut organism = population.evolve(|organisms| {
        organisms.iter_mut().for_each(|organism| {
            let mut score = 0.0;

            let result = organism.evaluate(&none);
            score += (result[0] - 0.0).powi(2);

            let result = organism.evaluate(&first);
            score += (result[0] - 1.0).powi(2);

            let result = organism.evaluate(&second);
            score += (result[0] - 1.0).powi(2);

            let result = organism.evaluate(&both);
            score += (result[0] - 0.0).powi(2);

            organism.fitness = Some(4.0 - score);
        })
    });
    println!("");
    println!("=========================================================================");
    println!("{:?}", organism);
    println!("=========================================================================");
    println!("");
    println!("0 xor 0: {:.3}", organism.evaluate(&none)[0]);
    println!("1 xor 0: {:.3}", organism.evaluate(&first)[0]);
    println!("0 xor 1: {:.3}", organism.evaluate(&second)[0]);
    println!("1 xor 1: {:.3}", organism.evaluate(&both)[0]);

    /*
    let mut pool = GenePool::new();
    let input1 = pool.create_input_node();
    let hidden1 = pool.create_hidden_node(50.0);
    let output1 = pool.create_output_node();
    let output2 = pool.create_output_node();
    pool.create_connection(input1, hidden1);
    pool.create_connection(input1, output1);
    pool.create_connection(hidden1, output1);
    pool.create_connection(hidden1, output2);

    let mut genome = pool.new_genome(&NewConnectionWeight::FIXED(1.0));

    let mutation_config = MutationConfig {
        change_weight_prob: 0.0,
        random_weight_dist: Normal::new(0.0, 0.3).unwrap(),
        shift_weight_prob: 0.0,
        shift_weight_dist: Normal::new(0.0, 1.0).unwrap(),
        add_node_prob: 1.0,
        add_connection_prob: 0.0,
        add_connection_retry_count: 5,
        new_connection_weight: NewConnectionWeight::FIXED(2.0),
    };
    genome.mutate(&mut pool, &mutation_config);

    serialize::store_genome("test.json", &genome, true).expect("Writing failed");


    let mut genome = serialize::read_genome("test.json").expect("Reading failed");
    let config = EvaluationConfig {
      activation: Activation::IDENTITY,
      bias: 0.0
    };
    println!("{:?}", genome.evaluate(&vec![1.0], &pool, &config));
    */
}
