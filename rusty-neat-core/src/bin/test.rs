use rusty_neat_core::{gene_pool::GenePool, population::Population};

use rusty_neat_interchange::io::FileType;

fn main() {
    let pool = GenePool::new_dense(3, 1);
    let mut population = Population::new(pool, "config.json").unwrap();
    let none = vec![0.0, 0.0, 1.0];
    let first = vec![1.0, 0.0, 1.0];
    let second = vec![0.0, 1.0, 1.0];
    let both = vec![1.0, 1.0, 1.0];
    let mut organism = population
        .evolve(
            |organisms| {
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
            },
            "xor_population",
        )
        .unwrap();
    println!("");
    println!("=========================================================================");
    println!("{:?}", organism);
    println!("=========================================================================");
    println!("");
    println!("0 xor 0: {:.3}", organism.evaluate(&none)[0]);
    println!("1 xor 0: {:.3}", organism.evaluate(&first)[0]);
    println!("0 xor 1: {:.3}", organism.evaluate(&second)[0]);
    println!("1 xor 1: {:.3}", organism.evaluate(&both)[0]);

    rusty_neat_interchange::organism::write(organism.clone(), "best.json", FileType::PrettyJSON)
        .unwrap();

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
