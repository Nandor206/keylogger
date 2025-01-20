use crossterm::{
    cursor, event::{self, KeyCode, KeyEvent, KeyModifiers}, style::StyledContent, terminal, ExecutableCommand
};
use std::fs::File;
use std::io::{self, Write};
use chrono::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    terminal::enable_raw_mode()?;
    // File creating
    let date = Local::now().format("%Y-%m-%d-%H:%M").to_string();
    let mut file = File::create(format!("{}.txt", date))?;

    loop {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Ok(event::Event::Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            })) = event::read()
            {
                match code {
                    KeyCode::Esc => {
                        println!("Exiting code");
                        break;
                    }
                    KeyCode::Char(c) => {
                        if modifiers.is_empty() {
                            // Normal key press
                            file.write_all(c.to_string().as_bytes())?;
                        } else {
                            // Special key combination
                            file.write_all(format!("\nSpecial: {} with modifiers: {:?}\n", c, modifiers).as_bytes())?;
                        }
                    }
                    KeyCode::Enter => {
                        file.write_all(b"\n")?;
                    }
                    _ => {
                        // Other special keys
                        file.write_all(format!("\nSpecial: {:?}\n", code).as_bytes())?;
                    }
                }
            }
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}
