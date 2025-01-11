// src/ui/app.rs
use std::error::Error;
use crossterm::event::{self, Event, KeyCode};
use crate::simulation::world::World;

pub struct App {
   pub world: World,
   pub selected_estajo_id: Option<u32>,
   pub should_quit: bool,
   pub is_paused: bool,
}

impl App {
   pub fn new(world: World) -> Self {
       Self {
           world,
           selected_estajo_id: None,
           should_quit: false,
           is_paused: false,
       }
   }

   pub fn tick(&mut self) -> Result<(), Box<dyn Error>> {
    if !self.is_paused {
        if let Err(e) = self.world.tick() {
            if e.kind() == std::io::ErrorKind::Other && e.to_string() == "All estajoj are dead!" {
                return Err(Box::new(e));
            }
            return Err(Box::new(e));
        }
    }
    Ok(())
}

   pub fn handle_input(&mut self) -> Result<(), Box<dyn Error>> {
       if event::poll(std::time::Duration::from_millis(100))? {
           if let Event::Key(key) = event::read()? {
               match key.code {
                   KeyCode::Char('q') | KeyCode::Esc => {
                       self.should_quit = true;
                   }
                   KeyCode::Char('p') => {
                       self.is_paused = !self.is_paused;
                   }
                   KeyCode::Left => {
                       self.select_previous_estajo();
                   }
                   KeyCode::Right => {
                       self.select_next_estajo();
                   }
                   _ => {}
               }
           }
       }
       Ok(())
   }

   fn select_next_estajo(&mut self) {
       let ids: Vec<u32> = self.world.estajoj.keys().copied().collect();
       if ids.is_empty() { return; }

       self.selected_estajo_id = Some(match self.selected_estajo_id {
           None => ids[0],
           Some(current) => {
               let pos = ids.iter().position(|&id| id == current)
                   .unwrap_or(0);
               ids[(pos + 1) % ids.len()]
           }
       });
   }

   fn select_previous_estajo(&mut self) {
       let ids: Vec<u32> = self.world.estajoj.keys().copied().collect();
       if ids.is_empty() { return; }

       self.selected_estajo_id = Some(match self.selected_estajo_id {
           None => ids[ids.len() - 1],
           Some(current) => {
               let pos = ids.iter().position(|&id| id == current)
                   .unwrap_or(0);
               if pos == 0 { ids[ids.len() - 1] } else { ids[pos - 1] }
           }
       });
   }
}