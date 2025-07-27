use std::env;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal, execute,
    style::Print
};
use std::io::{self, Write};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // User input part
    println!("Enter your custom text for the typing test:");
    
    let mut sample_text = String::new();
    io::stdin().read_line(&mut sample_text)?;
    sample_text = sample_text.trim().to_string();  // Remove trailing newline
    println!("\nType this: {}", sample_text);

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    let mut typed_text = String::new();
    let start_time = Instant::now();

    loop {
        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Esc => break,
                KeyCode::Enter => break,
                KeyCode::Char(c) => {
                    // Echo the typed character
                    execute!(stdout, Print(c))?;
                    typed_text.push(c);
                }
                KeyCode::Backspace => {
                    // Handle backspace (optional)
                    if !typed_text.is_empty() {
                        typed_text.pop();
                        execute!(stdout, Print("\x08 \x08"))?; // Erase last char
                    }
                }
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?;

    let elapsed = start_time.elapsed().as_secs_f64();
    let wpm = calculate_wpm(&typed_text, elapsed);
    let accuracy = calculate_accuracy(&sample_text, &typed_text);

    println!("\n\nTime: {:.2}s | WPM: {:.1} | Accuracy: {:.1}%", 
        elapsed, wpm, accuracy * 100.0);

    Ok(())
}

// Words-per-minute (5 chars = 1 word)
fn calculate_wpm(text: &str, time_secs: f64) -> f64 {
    let chars_per_min = (text.len() as f64 / time_secs) * 60.0;
    chars_per_min / 5.0
}

// Accuracy: % of correct characters
fn calculate_accuracy(expected: &str, typed: &str) -> f64 {
    let correct = expected.chars()
        .zip(typed.chars())
        .filter(|(e, t)| e == t)
        .count();
    correct as f64 / expected.len() as f64
}
