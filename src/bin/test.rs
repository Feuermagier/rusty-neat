use rusty_neat::gene_pool::{GenePool, NodeType};

fn main() {
  let mut pool = GenePool::new(0, 0);
  let input1 = pool.create_node(0.0, NodeType::Input(0));
  let hidden1 = pool.create_node(50.0, NodeType::Hidden);
  let output1 = pool.create_node(100.0, NodeType::Output(0));
  let output2 = pool.create_node(100.0, NodeType::Output(1));
  pool.create_connection(input1, hidden1);
  pool.create_connection(hidden1, output1);
  pool.create_connection(hidden1, output2);
  let mut genome = pool.new_genome();
  for i in 0..3 {
    println!("{:?}", genome.evaluate(&vec![2.0], i + 1, &pool));
  }
}