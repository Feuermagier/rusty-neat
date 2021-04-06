use std::rc::Rc;

use criterion::{criterion_group, criterion_main, Criterion};

use rusty_neat::{
    activation::Activation,
    gene_pool::{Connection, GenePool},
    genome::{EvaluationConfig, Genome, NewConnectionWeight},
};

fn evaluate_large_flat(criterion: &mut Criterion) {
    let pool = create_large_flat_pool();

    let mut input = Vec::with_capacity(500);

    for i in 0..500 {
        input.push(i as f64);
    }

    let mut genome = pool.new_genome(&NewConnectionWeight::FIXED(1.0));
    let config = EvaluationConfig {
        activation: Activation::IDENTITY,
        bias: 0.0,
    };
    criterion.bench_function("evaluate large flat network", |b| {
        b.iter(|| genome.evaluate(&input, &pool, &config))
    });
}

fn create_genome_from_large_pool(criterion: &mut Criterion) {
    let pool = create_large_flat_pool();
    criterion.bench_function("create genome from large pool", |b| {
        b.iter(|| pool.new_genome(&NewConnectionWeight::FIXED(1.0)))
    });
}

fn create_empty_genome(criterion: &mut Criterion) {
    criterion.bench_function("create empty genome", |b| b.iter(|| Genome::new()));
}

fn add_node_to_empty_genome(criterion: &mut Criterion) {
    let mut genome = Genome::new();
    criterion.bench_function("add node to empty genome", |b| {
        b.iter(|| genome.add_node(0))
    });
}

fn add_connection_to_minimal_genome(criterion: &mut Criterion) {
    let mut genome = Genome::new();
    genome.add_node(0);
    genome.add_node(1);
    let connection = Rc::from(Connection {
        from: 0,
        to: 1,
        innovation: 0,
    });
    criterion.bench_function("add connection to minimal genome", |b| {
        b.iter(|| genome.add_new_connection(connection, &NewConnectionWeight::FIXED(1.0)))
    });
}

fn create_connection_in_minimal_pool(criterion: &mut Criterion) {
    let mut pool = GenePool::new(Activation::IDENTITY, 0.0);
    let input = pool.create_input_node();
    let output = pool.create_output_node();
    criterion.bench_function("create connection in minimal pool", |b| {
        b.iter(|| pool.create_connection(input, output))
    });
}

fn get_connection_from_minimal_pool(criterion: &mut Criterion) {
    let mut pool = GenePool::new(Activation::IDENTITY, 0.0);
    let input = pool.create_input_node();
    let output = pool.create_output_node();
    pool.create_connection(input, output);
    criterion.bench_function("get connection from minimal pool", |b| {
        b.iter(|| pool.create_connection(input, output))
    });
}

fn create_large_flat_pool() -> GenePool {
    let mut pool = GenePool::new(Activation::IDENTITY, 0.0);

    for _ in 0..500 {
        pool.create_input_node();
    }

    for i in 0..500 {
        pool.create_output_node();

        for j in 0..500 {
            pool.create_connection(j, i + 500);
        }
    }

    pool
}

criterion_group!(evaluation, evaluate_large_flat);
criterion_group!(
    pool,
    create_genome_from_large_pool,
    create_connection_in_minimal_pool,
    get_connection_from_minimal_pool
);
criterion_group!(
    genome,
    create_empty_genome,
    add_node_to_empty_genome,
    add_connection_to_minimal_genome
);
criterion_main!(evaluation, pool, genome);
