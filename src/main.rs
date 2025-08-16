use ratatui::{DefaultTerminal, widgets::ListState};

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

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let app = App {
        should_exit: false,
        todo_list: TodoList {
            items: vec![],
            state: ListState::default(),
        },
    };

    let res = app.run(terminal);

    ratatui::restore();

    res
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }
}
