// src/storage/mod.rs


// src/storage/history.rs
use std::fs::{File, OpenOptions};
use std::io::BufWriter;
use std::io::Write;
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::models::event::Event;
use crate::simulation::parameters::SimulationParams;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimulationRecord {
    simulation_id: String,
    start_time: DateTime<Local>,
    parameters: SimulationParams,
    events: Vec<Event>,
}

pub struct HistoryStorage {
    current_simulation: SimulationRecord,
    file_writer: BufWriter<File>,
}

impl HistoryStorage {
    pub fn new(parameters: SimulationParams) -> std::io::Result<Self> {
        let simulation_id = Uuid::new_v4().to_string();
        let start_time = Local::now();
        let filename = format!(
            "simulation_{}.json",
            start_time.format("%Y%m%d_%H%M%S")
        );

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(filename)?;

        Ok(Self {
            current_simulation: SimulationRecord {
                simulation_id,
                start_time,
                parameters,
                events: Vec::new(),
            },
            file_writer: BufWriter::new(file),
        })
    }

    pub fn record_event(&mut self, event: Event) -> std::io::Result<()> {
        self.current_simulation.events.push(event.clone());
        Ok(())
    }

    pub fn save(&mut self) -> std::io::Result<()> {
        serde_json::to_writer_pretty(&mut self.file_writer, &self.current_simulation)?;
        self.file_writer.flush()?;
        Ok(())
    }

    pub fn get_recent_events(&self, count: usize) -> Vec<&Event> {
        self.current_simulation.events
            .iter()
            .rev()
            .take(count)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::event::{EventType, Need};
    
    #[test]
    fn test_create_storage() {
        let params = SimulationParams::default();
        let storage = HistoryStorage::new(params);
        assert!(storage.is_ok());
    }

    #[test]
    fn test_record_event() {
        let params = SimulationParams::default();
        let mut storage = HistoryStorage::new(params).unwrap();
        let event = Event::new(
            EventType::Need(Need::Food),
            "Test event".to_string()
        );
        assert!(storage.record_event(event).is_ok());
    }
}