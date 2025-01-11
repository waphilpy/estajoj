use serde::{Deserialize, Serialize};
use rand::prelude::*;
use super::event::{Event, EventType, Need, StateChange};
use super::Needs;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Sex {
   Male,
   Female
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Genetics {
   pub energy_factor: f32,
   pub satisfaction_factor: f32,
   pub influence_factor: f32,
}

impl Genetics {
   pub fn new() -> Self {
       let mut rng = thread_rng();
       Self {
           energy_factor: rng.gen_range(0.8..1.2),
           satisfaction_factor: rng.gen_range(0.8..1.2),
           influence_factor: rng.gen_range(0.8..1.2),
       }
   }

   pub fn mix_with(&self, other: &Genetics) -> Genetics {
       let mut rng = thread_rng();
       Self {
           energy_factor: (self.energy_factor + other.energy_factor) / 2.0 * rng.gen_range(0.9..1.1),
           satisfaction_factor: (self.satisfaction_factor + other.satisfaction_factor) / 2.0 * rng.gen_range(0.9..1.1),
           influence_factor: (self.influence_factor + other.influence_factor) / 2.0 * rng.gen_range(0.9..1.1),
       }
   }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Estajo {
   pub id: u32,
   pub name: String,
   pub sex: Sex,
   pub life: f32,
   pub needs: Needs,
   pub genetics: Genetics,
   pub history: Vec<Event>,
}

impl Estajo {
   pub fn new(id: u32, name: String) -> Self {
       let sex = if thread_rng().gen_bool(0.5) { Sex::Male } else { Sex::Female };
       Self {
           id,
           name,
           sex,
           life: 100.0,
           needs: Needs::new(),
           genetics: Genetics::new(),
           history: Vec::new(),
       }
   }

    pub fn is_alive(&self) -> bool {
        self.life > 0.0
    }

    pub fn take_damage(&mut self, amount: f32) {
        self.life = (self.life - amount).max(0.0);
        if !self.is_alive() {
            self.add_event(Event::new(
                EventType::StateChange(StateChange::Death),
                "Has died".to_string()
            ));
        }
    }

   pub fn reproduce_with(&self, partner: &Estajo) -> Option<Estajo> {
       if self.sex == partner.sex { 
           return None; 
       }
       
       Some(Estajo {
           id: thread_rng().gen(),
           name: format!("Child_{}_{}", self.id, partner.id),
           sex: if thread_rng().gen_bool(0.5) { Sex::Male } else { Sex::Female },
           life: 100.0,
           needs: Needs::new(),
           genetics: self.genetics.mix_with(&partner.genetics),
           history: Vec::new(),
       })
   }

   pub fn update_needs(&mut self) {
       self.needs.update();
       if self.needs.hunger < 20.0 {
           self.add_event(Event::new(
               EventType::Need(Need::Food),
               "Hungry".to_string()
           ));
       }
       if self.needs.ambition > 80.0 {
           self.add_event(Event::new(
               EventType::Need(Need::Ambition),
               "Ambitious".to_string()
           ));
       }
   }

   pub fn add_event(&mut self, event: Event) {
       self.history.push(event);
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_create_estajo() {
       let estajo = Estajo::new(1, "Test".to_string());
       assert_eq!(estajo.id, 1);
       assert_eq!(estajo.history.len(), 0);
   }

   #[test]
   fn test_reproduction() {
       let mut e1 = Estajo::new(1, "Parent1".to_string());
       let mut e2 = Estajo::new(2, "Parent2".to_string());
       
       e1.sex = Sex::Male;
       e2.sex = Sex::Female;

       let child = e1.reproduce_with(&e2);
       assert!(child.is_some());
       
       let child = child.unwrap();
       assert!(child.name.contains("Child"));
   }

   #[test]
   fn test_same_sex_reproduction() {
       let mut e1 = Estajo::new(1, "Parent1".to_string());
       let mut e2 = Estajo::new(2, "Parent2".to_string());
       
       e1.sex = Sex::Male;
       e2.sex = Sex::Male;

       let child = e1.reproduce_with(&e2);
       assert!(child.is_none());
   }

   #[test]
   fn test_needs_update() {
       let mut estajo = Estajo::new(1, "Test".to_string());
       let initial_hunger = estajo.needs.hunger;
       estajo.update_needs();
       assert!(estajo.needs.hunger < initial_hunger);
   }
}