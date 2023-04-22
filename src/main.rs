#![allow(dead_code)]
#![allow(unused)]
use std::io;
use termion::input::Keys;
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

fn main() {
    let stdout = io::stdout();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear();

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(50)].as_ref())
                .split(frame.size());
            let left_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(0)].as_ref())
                .split(chunks[0]);

            let block = Block::default()
                .title("termkeeper")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let upper_left_block = Block::default()
                .title("upper_left_block")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));
            let lower_left_block = Block::default()
                .title("lower_left_block")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::Black));

            let text = Span::raw("hello world");
            let p = Paragraph::new(text);

            frame.render_widget(block, chunks[1]);
            frame.render_widget(upper_left_block, left_chunks[0]);
            frame.render_widget(lower_left_block, left_chunks[1]);

            // frame.render_widget(p, chunks[1]);
        });
    }
}
