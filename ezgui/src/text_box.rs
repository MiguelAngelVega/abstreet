// Copyright 2018 Google LLC, licensed under http://www.apache.org/licenses/LICENSE-2.0

use keys::key_to_char;
use piston::input::{Button, Event, Key, PressEvent, ReleaseEvent};
use TextOSD;

// TODO right now, only a single line

pub struct TextBox {
    // TODO A rope would be cool.
    pub line: String,
    cursor_x: usize,
    shift_pressed: bool,
}

impl TextBox {
    pub fn new() -> TextBox {
        TextBox::new_prefilled(String::from(""))
    }

    pub fn new_prefilled(line: String) -> TextBox {
        TextBox {
            line,
            cursor_x: 0,
            shift_pressed: false,
        }
    }

    pub fn populate_osd(&self, osd: &mut TextOSD) {
        osd.add_line_with_cursor(self.line.clone(), self.cursor_x);
    }

    // Returns true if the user confirmed their entry.
    pub fn event(&mut self, ev: &Event) -> bool {
        // Done?
        if let Some(Button::Keyboard(Key::Return)) = ev.press_args() {
            return true;
        }

        // Key state tracking
        if let Some(Button::Keyboard(Key::LShift)) = ev.press_args() {
            self.shift_pressed = true;
        }
        if let Some(Button::Keyboard(Key::LShift)) = ev.release_args() {
            self.shift_pressed = false;
        }

        // Cursor movement
        if let Some(Button::Keyboard(Key::Left)) = ev.press_args() {
            if self.cursor_x > 0 {
                self.cursor_x -= 1;
            }
        }
        if let Some(Button::Keyboard(Key::Right)) = ev.press_args() {
            self.cursor_x = (self.cursor_x + 1).min(self.line.len());
        }

        // Backspace
        if let Some(Button::Keyboard(Key::Backspace)) = ev.press_args() {
            if self.cursor_x > 0 {
                self.line.remove(self.cursor_x - 1);
                self.cursor_x -= 1;
            }
        }

        // Insert
        if let Some(Button::Keyboard(key)) = ev.press_args() {
            if let Some(mut c) = key_to_char(key) {
                if !self.shift_pressed {
                    c = c.to_lowercase().next().unwrap();
                }
                self.line.insert(self.cursor_x, c);
                self.cursor_x += 1;
            }
        }
        false
    }
}
