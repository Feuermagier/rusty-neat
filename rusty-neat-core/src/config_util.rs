use rand_distr::Normal;
use serde::{Deserialize, Serialize};

pub(crate) fn assert_probability(value: f64, name: &str) -> Result<(), String> {
    if value >= 0.0 && value <= 1.0 {
        Ok(())
    } else {
        Err(name.to_owned() + " must be a probabilty (in [0,1])")
    }
}

pub(crate) fn assert_not_negative(value: f64, name: &str) -> Result<(), String> {
    if value >= 0.0 {
        Ok(())
    } else {
        Err(name.to_owned() + " must not be negative")
    }
}

pub(crate) fn assert_ratio(value: f64, name: &str) -> Result<(), String> {
    if value >= 0.0 && value <= 1.0 {
        Ok(())
    } else {
        Err(name.to_owned() + " must be a ratio (in [0,1])")
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct NormalDistribution {
    pub mean: f64,
    pub std_dev: f64,
}

impl NormalDistribution {
    pub(crate) fn to_dist(self) -> Normal<f64> {
        Normal::new(self.mean, self.std_dev).unwrap()
    }
}
