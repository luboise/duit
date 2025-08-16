use std::io;

mod stateful_list;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    layout::Rect,
    widgets::{List, ListItem, ListState},
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

    // Render state
    render_list_state: ListState,
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
        render_list_state: ListState::default(),
    };

    app.render_list_state.select(Some(0));

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

    fn try_exit(&mut self) {
        self.should_exit = true;
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.draw_list(frame, frame.area());
    }

    fn draw_list(&mut self, frame: &mut Frame, area: Rect) {
        let todos = List::new(self.todo_list.iter().map(|f| ListItem::new(f.name.clone())))
            .highlight_symbol(">> ");
        frame.render_stateful_widget(todos, area, &mut self.render_list_state);
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

    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.kind {
            KeyEventKind::Press => match event.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => self.try_exit(),
                KeyCode::Char('n') => {
                    if event.modifiers.intersects(KeyModifiers::CONTROL) {
                        self.render_list_state.select_next();
                    }
                }

                KeyCode::Char('p') => {
                    if event.modifiers.intersects(KeyModifiers::CONTROL) {
                        self.render_list_state.select_previous();
                    }
                }
                _ => (),
            },
            KeyEventKind::Repeat => (),
            KeyEventKind::Release => (),
        }
    }
}
/*

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

        // list.block(block).render(area, buf);

        /*
        // Put the paragraph into the block
        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
        */
    }
}
*/
