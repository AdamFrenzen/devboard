use std::process::Command;

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Insert,
}

pub struct App {
    pub title: String,
    pub mode: Mode,
    pub input: String,
    pub commands: Vec<String>,
    pub selected_index: usize,
    pub scroll_offset: u16,
    pub output: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            title: " devboard ".to_string(),
            mode: Mode::Insert,
            input: String::new(),
            commands: Vec::new(),
            // commands: vec![
            //     "cargo build",
            //     "cargo run",
            //     "cargo test",
            //     "nvim source",
            //     "nvim ~/.tmux.conf",
            //     "cargo build --release",
            //     "mkdir mydir",
            // ],
            selected_index: 0,
            scroll_offset: 0,
            output: String::new(),
        }
    }

    pub fn next_command(&mut self, view_width: u16) {
        if self.selected_index < self.commands.len().saturating_sub(1) {
            self.selected_index += 1;
            self.horizontal_scroll(view_width);
        }
    }

    pub fn previous_command(&mut self, view_width: u16) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.horizontal_scroll(view_width);
        }
    }

    pub fn selected_command(&self) -> &str {
        &self.commands[self.selected_index]
    }

    pub fn delete_selected_command(&mut self) {
        self.commands.remove(self.selected_index);
    }

    pub fn save_command(&mut self) {
        self.commands.push(self.input.to_string());
    }

    pub fn run_input(&mut self) {
        self.run_command(&self.input.to_string())
    }

    pub fn run_selected_command(&mut self) {
        self.run_command(&self.selected_command().to_string())
    }

    fn run_command(&mut self, command: &str) {
        let output = Command::new("sh").arg("-c").arg(command).output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                self.output = format!("{}{}", stdout, stderr);
            }
            Err(e) => {
                self.output = format!("Error running command: {}", e);
            }
        }
    }

    fn horizontal_scroll(&mut self, view_width: u16) {
        let mut x: u16 = 0;

        for (i, cmd) in self.commands.iter().enumerate() {
            // "[cmd]  " -> n chars "cmd" + 2 chars "[]" + 2 chars "  "
            let width = cmd.len() as u16 + 4;
            if i == self.selected_index {
                self.scroll_offset = x.saturating_sub(view_width / 7);
                break;
            }
            x += width;
        }
    }
}
