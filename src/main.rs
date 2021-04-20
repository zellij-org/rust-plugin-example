use zellij_tile::prelude::*;

use chrono::{DateTime, Local};
use std::{collections::VecDeque, fs::File, io::Write};

#[derive(Default)]
struct State {
    log: VecDeque<(String, DateTime<Local>)>,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self) {
        subscribe(&[EventType::ModeUpdate, EventType::KeyPress]);
    }

    fn update(&mut self, event: Event) {
        match event {
            Event::ModeUpdate(mode_info) => {
                let mode = format!("{:?}", mode_info.mode);
                self.log.push_front((mode, Local::now()));
            }
            Event::KeyPress(Key::Ctrl('l')) => self.log.clear(),
            Event::KeyPress(Key::Ctrl('w')) => {
                if let Ok(mut f) = File::create("mode-log.txt") {
                    for (mode, time) in self.log.iter().rev() {
                        writeln!(f, "{}: Entered {} Mode", time.format("%c"), mode).unwrap();
                    }
                }
            }
            _ => (),
        }
    }

    fn render(&mut self, _rows: usize, cols: usize) {
        for (mode, time) in &self.log {
            let mode = format!("Mode: {}", mode);
            let time = time.format("%T").to_string();
            let padding = " ".repeat(cols - mode.len() - time.len());
            println!("{}{}{}", mode, padding, time);
        }
    }
}
