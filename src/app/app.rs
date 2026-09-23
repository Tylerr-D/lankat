use crossterm::event::KeyCode;

pub struct App {
    pub messages: Vec<String>,
    pub peers: Vec<String>,
    pub input: String,
}

impl App {
    pub fn handle_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => return true,
            KeyCode::Char(c) => self.input.push(c),
            KeyCode::Backspace => { self.input.pop(); }
            KeyCode::Enter => self.send(),
            _ => {}
        }
        false
    }

    fn send(&mut self) {
        let text = self.input.trim().to_string();
        if !text.is_empty() {
            self.messages.push(format!("me:{text}"));
            self.input.clear();
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            messages: vec!["lankat:woaowaowoaoowoaoowoowoaoaowowoao".to_string()],
            peers: Vec::new(),
            input: String::new(),
        }
    }
}
