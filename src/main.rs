use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, List, ListState, Paragraph, Widget},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum TodoStatus {
    Todo,
    Completed,
}

#[derive(Debug)]
struct TodoItem {
    name: String,
    // info: String,
    // status: TodoStatus,
}

struct App {
    should_exit: bool,
    todo_list: Vec<TodoItem>,
}

fn main() -> io::Result<()> {
    color_eyre::install().expect("Unable to initialies color_eyre colours in the terminal.");
    let mut terminal = ratatui::init();

    let mut app = App {
        should_exit: false,
        todo_list: vec![
            TodoItem {
                name: "Do stuff".to_string(),
            },
            TodoItem {
                name: "Do stuff 2".to_string(),
            },
        ],
    };

    let res = app.run(&mut terminal);

    ratatui::restore();

    res
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.kind {
            KeyEventKind::Press => match event.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    self.should_exit = true;
                }
                _ => (),
            },
            KeyEventKind::Repeat => (),
            KeyEventKind::Release => (),
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let list = List::new(self.todo_list.iter().map(|f| stringify!(f)));

        list.block(block).render(area, buf);

        /*
        // Put the paragraph into the block
        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
        */
    }
}
