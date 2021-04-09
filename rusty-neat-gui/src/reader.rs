struct Genome {
  connections: Vec<Connection>,
  connection_mappings: 
}

struct Node {
  node_id: usize
}

struct Connection {
  innovation: usize,
  from: usize,
  to: usize,
  weight: f64,
  enabled: bool
}