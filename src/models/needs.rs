// src/models/needs.rs
use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Needs {
   pub hunger: f32,     // 0-100
   pub ambition: f32,   // 0-100
}

impl Needs {
   pub fn new() -> Self {
       Self {
           hunger: 100.0,
           ambition: rand::thread_rng().gen_range(30.0..70.0),
       }
   }

   pub fn update(&mut self) {
       self.hunger -= 2.0;
       self.hunger = self.hunger.max(0.0);
       
       self.ambition += rand::thread_rng().gen_range(-1.0..2.0);
       self.ambition = self.ambition.clamp(0.0, 100.0);
   }

   pub fn eat(&mut self, amount: f32) {
       self.hunger += amount;
       self.hunger = self.hunger.min(100.0);
   }
}