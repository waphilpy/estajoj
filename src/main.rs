// src/main.rs
use std::io;
use std::error::Error;
use crossterm::{
   terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
   execute,
};
use ratatui::{
   backend::CrosstermBackend,
   Terminal,
};
use estajoj::{
   simulation::{world::World, parameters::SimulationParams},
   ui::{app::App, tui},
};

fn main() -> Result<(), Box<dyn Error>> {
   // Setup terminal
   enable_raw_mode()?;
   let mut stdout = io::stdout();
   execute!(stdout, EnterAlternateScreen)?;
   let backend = CrosstermBackend::new(stdout);
   let mut terminal = Terminal::new(backend)?;

   // Create app
   let params = SimulationParams::default();
   let world = World::new(params)?;
   let mut app = App::new(world);

   // Main loop
   loop {
       terminal.draw(|f| tui::draw::<CrosstermBackend<io::Stdout>>(f, &app))?;
       
       app.handle_input()?;
       if app.should_quit {
           break;
       }
       
       if let Err(e) = app.tick() {
           if e.to_string() == "All estajoj are dead!" {
               break;
           }
           return Err(e);
       }
   }

   // Restore terminal
   disable_raw_mode()?;
   execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
   terminal.show_cursor()?;

   Ok(())
}