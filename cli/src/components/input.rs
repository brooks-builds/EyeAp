use anathema::component::*;
use anathema::prelude::*;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(State)]
pub struct InputState {
    input: Value<String>,
    cursor: Value<char>,
    cursor_pos: Value<usize>,
    block: Value<bool>,

    #[state_ignore]
    cursor_index: usize,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            input: String::new().into(),
            cursor: '|'.into(),
            cursor_pos: 0.into(),
            cursor_index: 0,
            block: false.into(),
        }
    }

    fn update_cursor_char(&mut self) {
        let c = match *self.cursor_pos.to_ref() == self.input.to_ref().width()
            || self.input.to_ref().is_empty()
        {
            true => {
                self.block.set(false);
                '|'
            }
            false => {
                self.block.set(true);
                self.input.to_ref()[self.cursor_index..]
                    .chars()
                    .next()
                    .unwrap()
            }
        };
        self.cursor.set(c);
    }

    pub fn insert(&mut self, c: char) {
        self.input.to_mut().insert(self.cursor_index, c);
        self.cursor_index += c.len_utf8();
        *self.cursor_pos.to_mut() += c.width().unwrap_or(0);
        self.update_cursor_char();
    }

    pub fn insert_str(&mut self, s: &str) {
        self.input.to_mut().insert_str(self.cursor_index, s);
        self.cursor_index += s.len();
        *self.cursor_pos.to_mut() += s.width();
        self.update_cursor_char();
    }

    pub fn move_left(&mut self) {
        if *self.cursor_pos.to_ref() == 0 {
            return;
        }
        let (i, c) = self.input.to_ref()[..self.cursor_index]
            .char_indices()
            .rev()
            .next()
            .unwrap();

        *self.cursor_pos.to_mut() -= c.width().unwrap_or(0);
        self.cursor_index = i;
        self.update_cursor_char();
    }

    pub fn move_right(&mut self) {
        let input = self.input.to_ref();
        if *self.cursor_pos.to_ref() == input.width() || input.is_empty() {
            return;
        }

        let (i, c) = input
            .char_indices()
            .skip_while(|(i, _)| *i <= self.cursor_index)
            .next()
            .unwrap_or_else(|| (input.len(), ' '));

        *self.cursor_pos.to_mut() += c.width().unwrap_or(0);
        self.cursor_index = i;
        drop(input);
        self.update_cursor_char();
    }

    pub fn _move_to_start(&mut self) {
        self.cursor_index = 0;
        self.cursor_pos.set(0);
        self.update_cursor_char();
    }

    pub fn _move_to_end(&mut self) {
        self.cursor_index = self.input.to_ref().len();
        self.cursor_pos.set(self.input.to_ref().width());
        self.update_cursor_char();
    }

    pub fn backspace(&mut self) {
        self.move_left();
        self.delete();
    }

    pub fn delete(&mut self) {
        self.input.to_mut().remove(self.cursor_index);
        self.update_cursor_char();
    }

    pub fn take(&mut self) -> String {
        self.cursor_pos.set(0);
        self.cursor_index = 0;
        self.cursor.set(' ');
        std::mem::take(&mut *self.input.to_mut())
    }
}

#[derive(State)]
pub struct IndexState {
    input: Value<InputState>,
    focus_color: Value<String>,
}

impl IndexState {
    pub fn new() -> Self {
        Self {
            input: InputState::new().into(),
            focus_color: Value::new("blue".to_owned()),
        }
    }
}

pub struct Input;

impl Component for Input {
    type Message = usize;
    type State = IndexState;

    fn on_key(
        &mut self,
        key: KeyEvent,
        state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_>,
    ) {
        match key.code {
            KeyCode::Char(c) => state.input.to_mut().insert(c),
            KeyCode::Tab => state.input.to_mut().insert_str("    "),
            KeyCode::Backspace => state.input.to_mut().backspace(),
            KeyCode::Enter => {
                let _input = state.input.to_mut().take();
            }
            KeyCode::Left => state.input.to_mut().move_left(),
            KeyCode::Right => state.input.to_mut().move_right(),
            KeyCode::Delete => state.input.to_mut().delete(),
            _ => {}
        }
    }

    fn on_focus(
        &mut self,
        state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_>,
    ) {
        state.focus_color.set("red".to_string());
    }

    fn on_blur(
        &mut self,
        state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_>,
    ) {
        state.focus_color.set("blue".to_string());
    }
}
