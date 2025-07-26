
use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sample text to type
    let sample_text = "The quick brown fox jumps over the lazy dog.";

    println!("Type this: {}", sample_text);
    println!("Press 'Esc' to quit or 'Enter' to finish.");

    terminal::enable_raw_mode()?; // Disable line buffering

    let start_time = Instant::now();
    let mut typed_text = String::new();

    loop {
        if let Event::Key(event) = event::read()? {
            match event.code {
                KeyCode::Esc => break, // Quit early
                KeyCode::Enter => break, // Finish typing
                KeyCode::Char(c) => {
                    print!("{}", c); // Echo typed char
                    typed_text.push(c);
                }
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()?; // Reset terminal

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

