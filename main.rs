use std::collections::HashMap;
use std::{io, fs};
use rand::seq::SliceRandom;
use rand::thread_rng;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode, KeyEventKind},
};
use ratatui::{prelude::*, widgets::*};

enum GameStatus {
    Playing,
    Won,
    Lost
}

struct GameState {
    word: String,
    wordmap: HashMap<u8, char>,
    found: Vec<bool>,
    previous_guesses: Vec<String>,
    bad_guesses: u8,
    current_input: String,
    message: String,
    status: GameStatus,
}

impl GameState {
    fn new(word: String) -> Self {
        let mut letters = Vec::new();
        let mut found = Vec::new();
        let mut previous_guesses = Vec::new();
        for (i, c) in word.chars(). enumerate() {
            letters.push((i as u8, c));
            found.push(false);
        }
        let wordmap: HashMap<u8, char> = letters.into_iter().collect();
        Self {
            word, 
            wordmap,
            found,
            previous_guesses,
            bad_guesses: 0,
            current_input: String::new(),
            message: String::from("Guess a letter or a word."),
            status: GameStatus::Playing,
        }
    }
    fn process_guess(&mut self) {
        let guess = self.current_input.trim().to_lowercase();
        self.current_input.clear();
        
        if guess.is_empty() {
            return;
        }

        if !guess.chars().all(char::is_alphabetic) {
            self.message = String::from("Invalid Input. Please try again.");
            return;
        }

        match guess.chars().count() {
            1 => {
                let c = guess.chars().next().unwrap();
                if self.wordmap.values().any(|&v| v == c) {
                    self.message = format!("Good guess! '{}' is in the word.", c);
                    let keys: Vec<usize> = self.wordmap.iter()
                        .filter(|&(_, &v)| v == c)
                        .map(|(&k, _)| k as usize)
                        .collect();
                    for k in keys {
                        self.found[k] = true;
                    }
                } else {
                    self.message = format!("Bad guess! Try again.");
                    self.bad_guesses += 1;
                    self.previous_guesses.push(guess);
                }
            }
            _ => {
                if guess == self.word {
                    self.found.iter_mut().for_each(|f| *f = true);
                    self.message = format!("The word was '{}'. You win!", self.word);
                } else {
                    self.message = format!("Bad guess! Try again.");
                    self.bad_guesses += 1;
                    self.previous_guesses.push(guess);
                }
            }
        }

        if self.bad_guesses >= 6 {
            self.status = GameStatus::Lost;
        } else if self.found.iter().all(|&f| f) {
            self.status = GameStatus::Won;
        }
    }
    fn word_display(&self) -> String {
        self.found.iter().enumerate()
            .map(|(i, &found)| {
                if found {self.wordmap[&(i as u8)]} else {'_'}
            })
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn hangman_art(bad_guesses: u8) -> &'static str {
    match bad_guesses {
        0 => "_______\n |       |\n |        \n |        \n |        \n_|___     ",
        1 => "_______\n |       |\n |       O\n |        \n |        \n_|___     ",
        2 => "_______\n |       |\n |       O\n |       |\n |        \n_|___     ",
        3 => "_______\n |       |\n |       O\n |      /|\n |        \n_|___     ",
        4 => "_______\n |       |\n |       O\n |      /|\\\n |        \n_|___     ",
        5 => "_______\n |       |\n |       O\n |      /|\\\n |      / \n_|___     ",
        _ => "_______\n |       |\n |       O\n |      /|\\\n |      / \\\n_|___     ",
    }
}

fn render(frame: &mut Frame, state: &GameState) {
    let area = frame.area();

    // Split screen into vertical sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);
    
    frame.render_widget(
        Paragraph::new("H A N G M A N")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM)),
        chunks[0],
    );

    frame.render_widget(
        Paragraph::new(hangman_art(state.bad_guesses))
            .alignment(Alignment::Center),
            chunks[1],
    );

    frame.render_widget(
        Paragraph::new(state.word_display())
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Word")),
            chunks[2],
    );

    frame.render_widget(
        Paragraph::new(state.previous_guesses.join(", "))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Previous guesses:")),
            chunks[3],
    );

    let input_block = match state.status {
        GameStatus::Playing => Block::default().borders(Borders::ALL).title("Your guess"),
        _ => Block::default().borders(Borders::ALL).title("Press 'q' to quit and 'n' for a new game."),
    };
    frame.render_widget(
        Paragraph::new(state.current_input.as_str()).block(input_block),
        chunks[4],
    );

    let msg =  match state.status {
        GameStatus::Won => format!("You win! The word was '{}'.", state.word),
        GameStatus::Lost => format!("You Lose! The word was '{}'", state.word),
        GameStatus::Playing => state.message.clone(),
    };
    frame.render_widget(
        Paragraph::new(msg).alignment(Alignment::Center),
        chunks[5]
    );
}
fn get_word() -> String {
    let contents = include_str!("../words.txt");
    let words: Vec<&str> = contents.lines().collect();
    words.choose(&mut thread_rng()).expect("Word list is empty.").to_string()
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = GameState::new(get_word());

    loop {
        terminal.draw(|frame| render(frame, &state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match state.status {
                GameStatus::Playing => match key.code {
                    KeyCode::Char(c) => state.current_input.push(c),
                    KeyCode::Backspace => {state.current_input.pop();}
                    KeyCode::Enter => state.process_guess(),
                    KeyCode::Esc => break,
                    _ => {}
                },
                _ => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('n') => state = GameState::new(get_word()),
                    _ => {}
                },
            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
