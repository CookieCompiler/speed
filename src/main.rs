use std::{path::PathBuf};
use clap::{Parser};
use std::fs;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::io::{stdout, Result};
use ratatui::layout::Alignment;
use ratatui::style::{Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{ Paragraph};
use crossterm::event::{poll, read, Event, KeyCode};
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    
    file_path: PathBuf,

    
    #[arg(short, long, default_value_t = 250)]
    speed: u16,
}

fn orp(word: &str) -> (String, String, String){

    let chars: Vec<char> = word.chars().collect();
    let len = chars.len();

    if len == 0{
        return (String::new(), String::new(), String::new());

    }

    let orp_index = if len == 1 {
        0
    } else {
        len / 3 
    };
    let left: String = chars[..orp_index].iter().collect();
    let center: String = chars[orp_index].to_string();
    let right: String = chars[orp_index + 1..].iter().collect();

    (left, center, right)
    

}

fn main() -> Result<()> {
    let args = Cli::parse();
    // println!("{:?}", args);
    let message: String = fs::read_to_string(&args.file_path)?;
    println!("{}", message);
    
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let text = std::fs::read_to_string(&args.file_path).expect("Не удалось прочитать файл");

    let words: Vec<&str> = text.split_whitespace().collect();

    if words.is_empty(){
        println!("file is empty");
        return Ok(());
    }

    let mut current_word_index = 0;

    let delay = Duration::from_millis(60_000 / args.speed as u64);

    let mut last_tick = Instant::now();

    


    loop {
        if current_word_index>= words.len() 
        {
            break;
        }

        let current_word=words[current_word_index];

        terminal.draw(|frame|{
        
        let area = frame.size();

        let (left, center, right) = orp(current_word);

        let left_part = Span::raw(left);
        let orp_part = Span::styled(center, Style::default().fg(Color::Red));
        let right_part = Span::raw(right);

        let line = Line::from(vec![left_part, orp_part, right_part]);

        let paragraph = Paragraph::new(line).alignment(Alignment::Center);
        
        frame.render_widget(paragraph, area);
        

        })?;

        if poll(Duration::from_millis(16))? {
        if let Event::Key(key) = read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }
        if last_tick.elapsed() >= delay {
            current_word_index += 1;
            last_tick = Instant::now();

        }
    }   

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    
    Ok(())

}
