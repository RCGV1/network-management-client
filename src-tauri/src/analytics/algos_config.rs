#![allow(dead_code)]

use std::{any::Any, collections::HashMap};

/// The activation struct is used to determine whether an algorithm should be run.
///
/// # Fields
///
/// * `run` - A boolean that determines whether an algorithm should be run.
#[derive(Debug)]
struct Activation {
    pub run: bool,
}

pub struct Params {
    pub params: HashMap<String, Box<dyn Any>>,
}

impl Params {
    pub fn new() -> Self {
        Params {
            params: HashMap::new(),
        }
    }

    pub fn add_param(&mut self, key: String, value: Box<dyn Any>) {
        self.params.insert(key, value);
    }

    pub fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        self.params.get(key).and_then(|v| v.downcast_ref::<T>())
    }
}

impl std::fmt::Debug for Params {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Params")
            .field("params", &self.params)
            .finish()
    }
}

impl Activation {
    /// Creates a new Activation object. Defaults to false.
    fn new() -> Self {
        Activation { run: false }
    }

    /// Sets whether an algorithm should be run.
    fn set(&mut self, run: bool) {
        self.run = run;
    }

    /// Gets whether an algorithm should be run.
    fn get(&self) -> bool {
        self.run
    }
}

/// Configuration for articulation point algorithm.
/// Currently only contains the activation struct.
/// In the future, this may contain other parameters
/// for the algorithm (if any).
#[derive(Debug)]
pub struct APConf {
    activation: Activation,
    pub params: Params,
}

/// Configuration for mincut algorithm.
/// Currently only contains the activation struct.
/// In the future, this may contain other parameters
/// for the algorithm (such as whether to include
/// the total weight of the mincut in the result).
#[derive(Debug)]
pub struct MinCutConf {
    activation: Activation,
    pub params: Params,
}

/// Configuration for diffusion centrality algorithm.
/// Currently contains the activation struct and T.
#[derive(Debug)]
pub struct DiffCentConf {
    activation: Activation,
    pub params: Params,
}

/// Configuration for most similar timeline algorithm.
/// Currently only contains the activation struct.
/// In the future, this **will** contain other parameters.
#[derive(Debug)]
pub struct MostSimTConf {
    activation: Activation,
    pub params: Params,
}

/// Configuration for predicted state algorithm.
/// Currently only contains the activation struct.
/// In the future, this **will** contain other parameters.
#[derive(Debug)]
pub struct PredStateConf {
    activation: Activation,
    pub params: Params,
}

#[derive(Debug)]
pub struct AlgoConfig {
    pub ap: APConf,
    pub mincut: MinCutConf,
    pub diff_cent: DiffCentConf,
    pub most_sim_t: MostSimTConf,
    pub pred_state: PredStateConf,
}

impl AlgoConfig {
    pub fn new() -> Self {
        AlgoConfig {
            ap: APConf {
                activation: Activation::new(),
                params: Params::new(),
            },
            mincut: MinCutConf {
                activation: Activation::new(),
                params: Params::new(),
            },
            diff_cent: DiffCentConf {
                activation: Activation::new(),
                params: Params::new(),
            },
            most_sim_t: MostSimTConf {
                activation: Activation::new(),
                params: Params::new(),
            },
            pred_state: PredStateConf {
                activation: Activation::new(),
                params: Params::new(),
            },
        }
    }

    /// Sets which algos to run.
    ///
    /// # Arguments
    ///
    /// * `self` - The algorithm configuration object.
    /// * `bitfield` - The bitfield of the algorithms.
    /// To run the articulation point algorithm, the bitfield is 0b00001.
    /// To run the mincut algorithm, the bitfield is 0b00010.
    /// To run the diffusion centrality algorithm, the bitfield is 0b00100.
    /// To run the most similar timeline algorithm, the bitfield is 0b01000.
    /// To run the predicted state algorithm, the bitfield is 0b10000.
    /// To run all algorithms, the bitfield is 0b11111.
    pub fn set_algos(&mut self, bitfield: u8) {
        self.set_ap(bitfield & 0b00001 != 0);
        self.set_mincut(bitfield & 0b00010 != 0);
        self.set_diff_cent(bitfield & 0b00100 != 0);
        self.set_most_sim_t(bitfield & 0b01000 != 0);
        self.set_pred_state(bitfield & 0b10000 != 0);
    }

    pub fn set_ap(&mut self, ap: bool) {
        self.ap.activation.set(ap);
    }

    pub fn set_mincut(&mut self, mincut: bool) {
        self.mincut.activation.set(mincut);
    }

    pub fn set_diff_cent(&mut self, diff_cent: bool) {
        self.diff_cent.activation.set(diff_cent);
    }

    pub fn set_most_sim_t(&mut self, most_sim_t: bool) {
        self.most_sim_t.activation.set(most_sim_t);
    }

    pub fn set_pred_state(&mut self, pred_state: bool) {
        self.pred_state.activation.set(pred_state);
    }

    pub fn get_ap_activation(&self) -> bool {
        self.ap.activation.get()
    }

    pub fn get_mincut_activation(&self) -> bool {
        self.mincut.activation.get()
    }

    pub fn get_diff_cent_activation(&self) -> bool {
        self.diff_cent.activation.get()
    }

    pub fn get_most_sim_t_activation(&self) -> bool {
        self.most_sim_t.activation.get()
    }

    pub fn get_pred_state_activation(&self) -> bool {
        self.pred_state.activation.get()
    }

    pub fn get_ap_params(&self) -> &Params {
        &self.ap.params
    }

    pub fn get_mincut_params(&self) -> &Params {
        &self.mincut.params
    }

    pub fn get_diff_cent_params(&self) -> &Params {
        &self.diff_cent.params
    }

    pub fn get_most_sim_t_params(&self) -> &Params {
        &self.most_sim_t.params
    }

    pub fn get_pred_state_params(&self) -> &Params {
        &self.pred_state.params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_algos() {
        let mut algo_config = AlgoConfig::new();
        algo_config.set_algos(0b00111);
        assert_eq!(algo_config.get_ap_activation(), true);
        assert_eq!(algo_config.get_mincut_activation(), true);
        assert_eq!(algo_config.get_diff_cent_activation(), true);
        assert_eq!(algo_config.get_most_sim_t_activation(), false);
        assert_eq!(algo_config.get_pred_state_activation(), false);

        println!("algo_config: {:?}", algo_config);
    }
}
