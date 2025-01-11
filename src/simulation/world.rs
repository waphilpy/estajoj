use std::collections::HashMap;
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;
use crate::models::estajo::Estajo;
use crate::models::event::{Action, Event, EventType, StateChange};
use super::parameters::SimulationParams;
use crate::models::estajo::Sex;
use crate::models::event::Need;
use crate::storage::history::HistoryStorage;


pub struct World {
    pub estajoj: HashMap<u32, Estajo>,
    rng: ThreadRng,
    current_tick: u32,
    params: SimulationParams,
    history: HistoryStorage,
}

impl World {
    pub fn new(params: SimulationParams) -> std::io::Result<Self> {
        let mut world = Self {
            estajoj: HashMap::new(),
            rng: thread_rng(),
            current_tick: 0,
            params: params.clone(),
            history: HistoryStorage::new(params)?,
        };
        world.initialize_population();
        
        Ok(world)
    }

    fn initialize_population(&mut self) {
        // Assurer un male et une femelle au minimum
        self.estajoj.insert(0, {
            let mut e = Estajo::new(0, "Estajo_0".to_string());
            e.sex = Sex::Male;
            e
        });
        self.estajoj.insert(1, {
            let mut e = Estajo::new(1, "Estajo_1".to_string());
            e.sex = Sex::Female;
            e
        });
        
        // Ajouter le reste de la population aléatoirement
        for id in 2..self.params.initial_population {
            self.estajoj.insert(id, Estajo::new(id, format!("Estajo_{}", id)));
        }
    }

    pub fn tick(&mut self) -> std::io::Result<Vec<Event>> {
        let mut events = Vec::new();
        self.current_tick += 1;

        // Vieillissement et mort
        let mut all_dead = true;
        for estajo in self.estajoj.values_mut() {
            estajo.take_damage(0.1);  // Vieillissement naturel
            if estajo.is_alive() {
                all_dead = false;
            }
        }

        if all_dead {
            // Sauvegarde finale avant de retourner l'erreur
            if let Err(e) = self.history.save() {
                eprintln!("Error saving final history: {}", e);
            }
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "All estajoj are dead!"
            ));
        }

        // Retire les morts
        self.estajoj.retain(|_, e| e.is_alive());

        // Mise à jour des besoins pour tous les estajoj
        for estajo in self.estajoj.values_mut() {
            estajo.update_needs();
        }

        // Interactions existantes
        if self.rng.gen::<f32>() < self.params.interaction_chance {
            if let Some(event) = self.random_interaction() {
                events.push(event);
            }
        }

        if self.rng.gen::<f32>() < self.params.reproduction_chance {
            if let Some(event) = self.try_reproduction() {
                events.push(event);
            }
        }

        // Gestion de la faim
        if self.rng.gen::<f32>() < self.params.hunger_tick_chance {
            if let Some(event) = self.try_feeding() {
                events.push(event);
            }
        }

        // Gestion des ambitions
        if self.rng.gen::<f32>() < self.params.ambition_tick_chance {
            if let Some(event) = self.process_ambitions() {
                events.push(event);
            }
        }

        // Enregistrer tous les événements
        for event in &events {
            self.history.record_event(event.clone())?;
        }

        // Sauvegarde périodique
        if self.current_tick % 10 == 0 {  // Sauvegarde tous les 10 ticks
            if let Err(e) = self.history.save() {
                eprintln!("Error saving history: {}", e);
            }
        }

        Ok(events)
    }

    fn try_feeding(&mut self) -> Option<Event> {
        let hungry_ids: Vec<u32> = self.estajoj.iter()
            .filter(|(_, e)| e.needs.hunger < 30.0)
            .map(|(id, _)| *id)
            .collect();

        if let Some(&id) = hungry_ids.get(self.rng.gen_range(0..hungry_ids.len().max(1))) {
            let estajo = self.estajoj.get_mut(&id)?;
            estajo.needs.eat(30.0);
            Some(Event::new(
                EventType::Need(Need::Food),
                format!("Estajo_{} ate", id)
            ))
        } else {
            None
        }
    }

    fn process_ambitions(&mut self) -> Option<Event> {
        let ambitious_ids: Vec<u32> = self.estajoj.iter()
            .filter(|(_, e)| e.needs.ambition > 70.0)
            .map(|(id, _)| *id)
            .collect();

        if let Some(&id) = ambitious_ids.get(self.rng.gen_range(0..ambitious_ids.len().max(1))) {
            //let estajo = self.estajoj.get_mut(&id)?;
            let target_id = self.rng.gen_range(0..self.estajoj.len() as u32);
            
            Some(Event::new(
                EventType::Need(Need::Ambition),
                format!("Estajo_{} shows ambition towards Estajo_{}", id, target_id)
            ))
        } else {
            None
        }
    }

    fn random_interaction(&mut self) -> Option<Event> {
        let ids: Vec<u32> = self.estajoj.keys().cloned().collect();
        if ids.len() < 2 {
            return None;
        }

        let initiator_id = *ids.get(self.rng.gen_range(0..ids.len()))?;
        let target_id = *ids.iter()
            .filter(|&&id| id != initiator_id)
            .nth(self.rng.gen_range(0..ids.len()-1))?;

        let action = match self.rng.gen_range(0..3) {
            0 => Action::Help,
            1 => Action::Hurt,
            _ => Action::Plot,
        };

        Some(Event::new(
            EventType::Action(action),
            format!("Estajo_{} -> Estajo_{}", initiator_id, target_id)
        ))
    }

    fn try_reproduction(&mut self) -> Option<Event> {
        let ids: Vec<u32> = self.estajoj.keys().cloned().collect();
        if ids.len() < 2 { return None; }

        let id1 = *ids.get(self.rng.gen_range(0..ids.len()))?;
        let id2 = *ids.iter()
            .filter(|&&id| id != id1)
            .nth(self.rng.gen_range(0..ids.len()-1))?;

        let parent1 = self.estajoj.get(&id1)?;
        let parent2 = self.estajoj.get(&id2)?;

        if let Some(child) = parent1.reproduce_with(parent2) {
            //let child_id = child.id;
            self.estajoj.insert(child.id, child);
            Some(Event::new(
                EventType::StateChange(StateChange::Reproduction),
                format!("New estajo born from {} and {}", id1, id2)
            ))
        } else {
            None
        }
    }

    pub fn run_simulation(&mut self) -> std::io::Result<Vec<Event>> {
        let mut all_events = Vec::new();
        
        while self.current_tick < self.params.simulation_duration {
            all_events.extend(self.tick()?);
        }

        // Sauvegarder l'historique à la fin
        self.history.save()?;

        Ok(all_events)
    }

    pub fn get_recent_events(&self, count: usize) -> Vec<&Event> {
        self.history.get_recent_events(count)
    }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_world_creation() {
       let params = SimulationParams::default();
       let world = World::new(params).unwrap();  // Ajout de unwrap()
       assert_eq!(world.estajoj.len(), 10);
   }

   #[test]
   fn test_simulation_runs() {
       let params = SimulationParams {
           interaction_chance: 1.0,
           reproduction_chance: 0.1,
           simulation_duration: 10,
           initial_population: 5,
           hunger_tick_chance: 0.5,
           ambition_tick_chance: 0.2,
       };
       let mut world = World::new(params).unwrap();  // Ajout de unwrap()
       let events = world.run_simulation().unwrap();  // Ajout de unwrap()
       assert!(!events.is_empty());
       assert_eq!(world.current_tick, 10);
   }

   #[test]
   fn test_random_interaction() {
       let params = SimulationParams::default();
       let mut world = World::new(params).unwrap();  // Ajout de unwrap()
       let event = world.random_interaction();
       assert!(event.is_some());
   }

   #[test]
   fn test_reproduction() {
       let params = SimulationParams {
           interaction_chance: 0.0,
           reproduction_chance: 1.0,
           simulation_duration: 10,
           initial_population: 2,
           hunger_tick_chance: 0.5,
           ambition_tick_chance: 0.2,
       };
       let mut world = World::new(params).unwrap();  // Ajout de unwrap()
       let events = world.run_simulation().unwrap();  // Ajout de unwrap()
       
       assert!(events.iter().any(|e| matches!(
           e.event_type,
           EventType::StateChange(StateChange::Reproduction)
       )));
       
       assert!(world.estajoj.len() > 2);
   }

   #[test]
   fn test_reproduction_same_sex() {
       let params = SimulationParams {
           interaction_chance: 0.0,
           reproduction_chance: 1.0,
           simulation_duration: 10,
           initial_population: 2,
           hunger_tick_chance: 0.5,
           ambition_tick_chance: 0.2,
       };
       let mut world = World::new(params).unwrap();  // Ajout de unwrap()
       
       let mut iter = world.estajoj.values_mut();
       if let Some(first) = iter.next() {
           first.sex = Sex::Male;
       }
       if let Some(second) = iter.next() {
           second.sex = Sex::Female;
       }
       
       let events = world.run_simulation().unwrap();  // Ajout de unwrap()
       assert!(events.iter().any(|e| matches!(
           e.event_type,
           EventType::StateChange(StateChange::Reproduction)
       )));
       assert!(world.estajoj.len() > 2);
   }

   #[test]
   fn test_feeding() {
       let params = SimulationParams {
           hunger_tick_chance: 1.0,
           ..Default::default()
       };
       let mut world = World::new(params).unwrap();  // Ajout de unwrap()
       
       if let Some(estajo) = world.estajoj.values_mut().next() {
           estajo.needs.hunger = 10.0;
       }

       let event = world.try_feeding();
       assert!(event.is_some());
       assert!(matches!(event.unwrap().event_type, EventType::Need(Need::Food)));
   }

   #[test]
   fn test_ambitions() {
       let params = SimulationParams {
           ambition_tick_chance: 1.0,
           ..Default::default()
       };
       let mut world = World::new(params).unwrap();  // Ajout de unwrap()
       
       if let Some(estajo) = world.estajoj.values_mut().next() {
           estajo.needs.ambition = 90.0;
       }

       let event = world.process_ambitions();
       assert!(event.is_some());
       assert!(matches!(event.unwrap().event_type, EventType::Need(Need::Ambition)));
   }
}