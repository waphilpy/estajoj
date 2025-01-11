// src/simulation/parameters.rs
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationParams {
    pub interaction_chance: f32,
    pub reproduction_chance: f32,
    pub hunger_tick_chance: f32,    
    pub ambition_tick_chance: f32,  
    pub simulation_duration: u32,
    pub initial_population: u32,
}

impl Default for SimulationParams {
    fn default() -> Self {
        Self {
            interaction_chance: 0.35,
            reproduction_chance: 0.2,
            hunger_tick_chance: 0.2, 
            ambition_tick_chance: 0.2,
            simulation_duration: 100,
            initial_population: 10,
        }
    }
}