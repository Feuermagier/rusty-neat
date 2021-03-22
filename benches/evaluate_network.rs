use criterion::{black_box, criterion_group, criterion_main, Criterion};

use rusty_neat::{gene_pool::{GenePool, NodeType}, genome::Genome};

fn evaluate_large_flat(criterion: &mut Criterion) {
  let pool = create_large_flat_pool();

  let mut input = Vec::with_capacity(500);

  for i in 0..500 {
    input.push(i as f64);
  }

  let mut genome = pool.new_genome();
  criterion.bench_function("evaluate large flat network", |b| b.iter(|| genome.evaluate(&input, 1, &pool)));
}

fn create_genome_from_large_pool(criterion: &mut Criterion) {
  let pool = create_large_flat_pool();
  criterion.bench_function("create genome from large pool", |b| b.iter(|| pool.new_genome()));
}

fn create_empty_genome(criterion: &mut Criterion) {
  criterion.bench_function("create empty genome", |b| b.iter(|| Genome::new()));
}

fn add_node_to_empty_genome(criterion: &mut Criterion) {
  let mut genome = Genome::new();
  criterion.bench_function("add node to empty genome", |b| b.iter(|| genome.add_node(0)));
}

fn add_connection_to_minimal_genome(criterion: &mut Criterion) {
  let mut genome = Genome::new();
  genome.add_node(0);
  genome.add_node(1);
  criterion.bench_function("add connection to minimal genome", |b| b.iter(|| genome.add_connection(0, 1, 0)));
}

fn create_large_flat_pool() -> GenePool {
  let mut pool = GenePool::new(0, 0);
  let input1 = pool.create_node(0.0, NodeType::Input(0));
  let hidden1 = pool.create_node(50.0, NodeType::Hidden);
  let output1 = pool.create_node(100.0, NodeType::Output(0));
  let output2 = pool.create_node(100.0, NodeType::Output(1));
  pool.create_connection(input1, hidden1);
  pool.create_connection(hidden1, output1);
  pool.create_connection(hidden1, output2);

  for i in 0..500 {
    pool.create_node(0.0, NodeType::Input(i));
  }

  for i in 0..500 {
    pool.create_node(100.0, NodeType::Output(i));

    for j in 0..500 {
      pool.create_connection(j, i + 500);
    }

  }

  pool
}

criterion_group!(evaluation, evaluate_large_flat);
criterion_group!(modification, create_genome_from_large_pool, create_empty_genome, add_node_to_empty_genome, add_connection_to_minimal_genome);
criterion_main!(evaluation, modification);