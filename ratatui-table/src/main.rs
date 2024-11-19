use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, HighlightSpacing, Paragraph, Row, Table, TableState, Widget},
    DefaultTerminal,
    Frame
};

fn main() -> Result<(), io::Error> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
struct Item {
    user: &'static str,
    mail: &'static str,
    timezone: &'static str,
}

const ITEMS: [Item; 6] = [
    Item {
        user: "fdehau",
        mail: "Florian Dehau <fdehau@users.noreply.github.com",
        timezone: "UTC+1",
    },
    Item {
        user: "joshka",
        mail: "Josh McKinney <joshka@users.noreply.github.com>",
        timezone: "UTC-8",
    },
    Item {
        user: "kdheepak",
        mail: "Dheepak Krishnamurthy <me@kdheepak.com>",
        timezone: "UTC-5",
    },
    Item {
        user: "mindoodoo",
        mail: "Leon Sautour <minindoo@users.noreply.github.com>",
        timezone: "UTC+1",
    },
    Item {
        user: "orhun",
        mail: "Orhun Parmaksız <orhun@archlinux.org>",
        timezone: "UTC+3",
    },
    Item {
        user: "Valentin271",
        mail: "Valentin271 <36198422+Valentin271@users.noreply.github.com>",
        timezone: "UTC+1",
    },
];


#[derive(Debug, Default)]
pub struct App {
    table_state: usize,
    items: Vec<Item>,
    exit: bool,
}

impl App  {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
       
        let row = ITEMS.iter().map(|item| { 
            Row::new([
                item.user.into(),
                item.mail.dim().into(),
                Line::from(item.timezone).alignment(Alignment::Right),
            ])
        });

        let table = Table::new(
            row,
            [
                Constraint::Length(11),
                Constraint::Length(29),
                Constraint::Length(5),
            ],
        )
        .header(Row::new(["User", "Email", "TZ"]).bold().underlined().blue())
        .highlight_spacing(HighlightSpacing::Always)
        .highlight_style(Style::new().reversed().bold())
        .highlight_symbol(">");
    
        let mut state = TableState::new().with_selected(Some(self.table_state));
        
        let content = Block::bordered().title("Mails");
        let inner = content.inner(frame.area());

        frame.render_widget(content, frame.area());
        frame.render_stateful_widget(table, inner, &mut state);
    }

    fn select_previus(&mut self) {
        if self.table_state == 0 {
            self.table_state = ITEMS.len() - 1;
        } else {
            self.table_state -= 1;
        }
    }

    fn select_next(&mut self) {
        if self.table_state == ITEMS.len() - 1 {
            self.table_state = 0;
        } else {
            self.table_state += 1;
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    // ANCHOR: handle_key_event fn
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            KeyCode::Up => self.select_previus(),
            KeyCode::Down => self.select_next(),
            _ => {}
        }
    }
}