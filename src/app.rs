#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

pub struct App {
    pub title: String,
    pub commands: Vec<&'static str>,
    pub selected_index: usize,
    pub scroll_offset: u16,
    pub input: String,
    pub mode: Mode,
}

impl App {
    pub fn new() -> Self {
        Self {
            title: " dashboard ".to_string(),
            commands: vec![
                "cargo build",
                "cargo run",
                "cargo test",
                "nvim source",
                "nvim ~/.tmux.conf",
                "cargo build --release",
                "mkdir mydir",
            ],
            selected_index: 0,
            scroll_offset: 0,
            input: String::new(),
            mode: Mode::Normal,
        }
    }

    pub fn next(&mut self, view_width: u16) {
        if self.selected_index < self.commands.len().saturating_sub(1) {
            self.selected_index += 1;
            self.center_selected(view_width);
        }
    }

    pub fn previous(&mut self, view_width: u16) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.center_selected(view_width);
        }
    }

    pub fn selected_command(&self) -> &str {
        self.commands[self.selected_index]
    }

    pub fn center_selected(&mut self, view_width: u16) {
        let mut x: u16 = 0;

        for (i, cmd) in self.commands.iter().enumerate() {
            let width = cmd.len() as u16 + 8; // "[cmd]" + padding
            if i == self.selected_index {
                // scroll_offset is where this command should be centered
                self.scroll_offset = x.saturating_sub(view_width / 2).max(0);
                break;
            }
            x += width;
        }
    }
}
