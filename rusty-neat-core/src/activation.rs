use serde::{Deserialize, Serialize};
use std::f64;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Activation {
    Identity,
    Sigmoid,
    Relu,
}

impl Activation {
    pub fn function(&self) -> fn(f64) -> f64 {
        match self {
            Activation::Identity => identity,
            Activation::Sigmoid => sigmoid,
            Activation::Relu => relu,
        }
    }
}

fn identity(x: f64) -> f64 {
    x
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn relu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        0.0
    }
}
