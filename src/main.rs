use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyEvent, KeyEventKind},
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, ListState, Paragraph, Widget},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum TodoStatus {
    Todo,
    Completed,
}

#[derive(Debug)]
struct TodoItem {
    todo: String,
    info: String,
    status: TodoStatus,
}

struct TodoList {
    items: Vec<TodoItem>,
    state: ListState,
}

struct App {
    should_exit: bool,
    todo_list: TodoList,
}

fn main() -> io::Result<()> {
    color_eyre::install().expect("Unable to initialies color_eyre colours in the terminal.");
    let mut terminal = ratatui::init();

    let mut app = App {
        should_exit: false,
        todo_list: TodoList {
            items: vec![],
            state: ListState::default(),
        },
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
        //
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        if event.code == event::KeyCode::Esc {
            self.should_exit = true;
        };
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

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            // self.counter.to_string().yellow(),
            "bruh".yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
