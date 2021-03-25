use rusty_neat::gene_pool::{GenePool};

fn main() {
  let mut pool = GenePool::new(0, 0);
  let input1 = pool.create_input_node();
  let hidden1 = pool.create_hidden_node(50.0);
  let output1 = pool.create_output_node();
  let output2 = pool.create_output_node();
  pool.create_connection(input1, hidden1);
  pool.create_connection(hidden1, output1);
  pool.create_connection(hidden1, output2);
  let mut genome = pool.new_genome();
  for i in 0..3 {
    println!("{:?}", genome.evaluate(&vec![2.0], i + 1, &pool));
  }

  let serialized = serde_json::to_string(&pool).unwrap();
  println!("serialized = {}", serialized);

  let mut deserialized: GenePool = serde_json::from_str(&serialized).unwrap();
  deserialized.rebuild_connection_mappings();
  println!("deserialized = {:?}", deserialized);
}