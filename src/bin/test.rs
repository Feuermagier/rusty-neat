use rusty_neat::{activation::Activation, gene_pool::{GenePool}, serialize};

fn main() {
  let mut pool = GenePool::new(Activation::IDENTITY, 0.0);
  let input1 = pool.create_input_node();
  let hidden1 = pool.create_hidden_node(50.0);
  let output1 = pool.create_output_node();
  let output2 = pool.create_output_node();
  pool.create_connection(input1, hidden1);
  pool.create_connection(hidden1, output1);
  pool.create_connection(hidden1, output2);

  serialize::store_genome("test.json", &pool.new_genome(), true).expect("Writing failed");


  let mut genome = serialize::read_genome("test.json").expect("Reading failed");
  println!("{:?}", genome.evaluate(&vec![2.0], 1, &pool));
}