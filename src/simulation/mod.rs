// src/simulation/mod.rs
pub mod world;
pub mod parameters;

// src/simulation/parameters.rs
pub struct SimulationParams {
   pub interaction_chance: f32,
   pub simulation_duration: u32,
   pub initial_population: u32,
}

impl Default for SimulationParams {
   fn default() -> Self {
       Self {
           interaction_chance: 0.3,
           simulation_duration: 100,
           initial_population: 10,
       }
   }
}
