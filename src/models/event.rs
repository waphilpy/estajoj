use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt::{self};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Event {
   pub timestamp: DateTime<Local>,
   pub event_type: EventType,
   pub details: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum EventType {
   Action(Action),
   StateChange(StateChange),
   Need(Need),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Action {
   Help,
   Hurt,
   Plot,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Need {
   Food,
   Reproduction,
   Ambition,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum StateChange {
    EnergyUpdate,
    SatisfactionUpdate,
    InfluenceUpdate,
    Reproduction,
    Death,
}

impl Event {
   pub fn new(event_type: EventType, details: String) -> Self {
       Self {
           timestamp: Local::now(),
           event_type,
           details,
       }
   }
}

impl fmt::Display for Event {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       let time = self.timestamp.format("%H:%M:%S");
       match &self.event_type {
           EventType::Action(action) => write!(f, "{} {} {}", time, action, self.details),
           EventType::StateChange(change) => write!(f, "{} {} {}", time, change, self.details),
           EventType::Need(need) => write!(f, "{} {} {}", time, need, self.details),
       }
   }
}

impl fmt::Display for Action {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
           Action::Help => write!(f, "HELP"),
           Action::Hurt => write!(f, "HURT"),
           Action::Plot => write!(f, "PLOT"),
       }
   }
}

impl fmt::Display for StateChange {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
           StateChange::EnergyUpdate => write!(f, "ENERGY"),
           StateChange::SatisfactionUpdate => write!(f, "SATISFACTION"),
           StateChange::InfluenceUpdate => write!(f, "INFLUENCE"),
           StateChange::Reproduction => write!(f, "REPRODUCTION"),
           StateChange::Death => write!(f, "DEAD"),
       }
   }
}

impl fmt::Display for Need {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       match self {
           Need::Food => write!(f, "FOOD"),
           Need::Reproduction => write!(f, "REPRODUCTION"),
           Need::Ambition => write!(f, "AMBITION"),
       }
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_create_event() {
       let event = Event::new(
           EventType::Need(Need::Food),
           "Hungry".to_string()
       );
       assert!(matches!(event.event_type, EventType::Need(Need::Food)));
   }
}