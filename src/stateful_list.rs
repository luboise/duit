pub use ratatui::widgets::ListState;
struct StatefulList<T> {
    state: ListState,
    list: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(list: Vec<T>) -> Self {
        Self {
            state: ListState::default(),
            list,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.list.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.list.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
