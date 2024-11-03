use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::time::Duration;
use std::{
    error::Error,
    io::{self, Write},
};

use crate::common::app_builder::{AppBuilder, Arbor};

pub struct Repl {
    arbor: Arbor,
    input: String,
    selected_suggestion: usize,
}

impl Repl {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            arbor: Arbor::build().await?,
            input: "".to_string(),
            selected_suggestion: 0,
        })
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let mut stdout = io::stdout();

        terminal::enable_raw_mode()?;
        execute!(stdout, terminal::EnterAlternateScreen)?;

        loop {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(event) = event::read()? {
                    match event.code {
                        KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                            break
                        }
                        KeyCode::Char(' ') => {
                            println!("nonono");
                        }
                        KeyCode::Char(c) => {
                            self.input.push(c);
                            self.selected_suggestion = 0;
                        }
                        KeyCode::Backspace => {
                            self.input.pop();
                            self.selected_suggestion = 0;
                        }
                        KeyCode::Enter => {
                            self.arbor
                                .autocomplete
                                .insert_word(self.input.clone())
                                .await?;
                            self.input = "".to_string();
                            self.selected_suggestion = 0;
                        }
                        KeyCode::Up => {
                            if self.selected_suggestion > 0 {
                                self.selected_suggestion -= 1;
                            }
                        }
                        KeyCode::Down => {
                            self.selected_suggestion += 1;
                        }
                        KeyCode::Tab => {
                            if let Some(suggestion) = self
                                .arbor
                                .autocomplete
                                .suggest_word(self.input.as_str())
                                .await?
                                .get(self.selected_suggestion)
                            {
                                self.input = suggestion.to_string()
                            }

                            self.selected_suggestion = 0;
                        }
                        KeyCode::Esc => break,
                        _ => panic!("Unknown keystroke!"),
                    }
                }
            }

            let suggestions = self
                .arbor
                .autocomplete
                .suggest_word(self.input.as_str())
                .await?;
            let max_index = suggestions.len().saturating_sub(1);
            if self.selected_suggestion > max_index {
                self.selected_suggestion = max_index;
            }

            execute!(stdout, Clear(ClearType::All))?;

            execute!(
                stdout,
                cursor::MoveTo(0, 0),
                Print(format!("> {}", self.input))
            )?;

            for (i, suggestion) in suggestions.iter().enumerate() {
                if i == self.selected_suggestion {
                    execute!(
                        stdout,
                        cursor::MoveTo(2, (i + 1) as u16),
                        SetForegroundColor(Color::Green),
                        Print(suggestion),
                        ResetColor
                    )?;
                } else {
                    execute!(
                        stdout,
                        cursor::MoveTo(2, (i + 1) as u16),
                        SetForegroundColor(Color::DarkGrey),
                        Print(suggestion),
                        ResetColor
                    )?;
                }
            }

            execute!(stdout, cursor::MoveTo(2 + self.input.len() as u16, 0))?;

            stdout.flush()?;
        }

        execute!(stdout, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        Ok(())
    }
}
