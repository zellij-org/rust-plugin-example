use ansi_term::{Style, Colour::Fixed};
use zellij_tile::prelude::*;

use std::collections::{HashMap, BTreeMap};

#[derive(Default)]
struct State {
    mode_log: HashMap<String, usize>,
    tabs: Vec<String>,
    test_runs: usize,
    userspace_configuration: BTreeMap<String, String>,
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.userspace_configuration = configuration;
        // we need the ReadApplicationState permission to receive the ModeUpdate and TabUpdate
        // events
        // we need the RunCommands permission to run "cargo test" in a floating window
        request_permission(&[PermissionType::ReadApplicationState, PermissionType::RunCommands]);
        subscribe(&[EventType::ModeUpdate, EventType::TabUpdate, EventType::Key]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::ModeUpdate(mode_info) => {
                let mode = format!("{:?}", mode_info.mode);
                let count = self.mode_log.entry(mode).or_insert(0);
                *count += 1;
                should_render = true;
            }
            Event::TabUpdate(tab_info) => {
                self.tabs = tab_info.iter().map(|t| t.name.clone()).collect();
                should_render = true;
            }
            Event::Key(key) => {
                if let Key::Char('n') = key {
                    self.test_runs += 1;
                    open_command_pane_floating(CommandToRun {
                        path: "cargo".into(),
                        args: vec!["test".to_owned()],
                        cwd: None,
                    });
                }
            }
            _ => (),
        };
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        let colored_rows = color_bold(CYAN, &rows.to_string());
        let colored_cols = color_bold(CYAN, &cols.to_string());
        println!("");
        println!("I have {} rows and {} columns", colored_rows, colored_cols);
        println!("");
        println!("{} {:#?}", color_bold(GREEN, "I was started with the following user configuration:"), self.userspace_configuration);
        println!("");
        println!("{}", color_bold(GREEN, "Modes:"));
        for (mode, count) in &self.mode_log {
            let count = color_bold(ORANGE, &count.to_string());
            println!("{} -> Changed {} times", mode, count);
        }
        println!("");
        let current_tabs = color_bold(GREEN, "Current Tabs:");
        let comma = color_bold(ORANGE, ", ");
        println!("{} {}", current_tabs, self.tabs.join(&comma));
        println!("");
        if self.test_runs > 0 {
            let test_run_count = color_bold(CYAN, &self.test_runs.to_string());
            println!("Ran tests {} times!", test_run_count);
        }
    }
}

pub const CYAN: u8 = 51;
pub const GRAY_LIGHT: u8 = 238;
pub const GRAY_DARK: u8 = 245;
pub const WHITE: u8 = 15;
pub const BLACK: u8 = 16;
pub const RED: u8 = 124;
pub const GREEN: u8 = 154;
pub const ORANGE: u8 = 166;

fn color_bold(color: u8, text: &str) -> String {
    format!("{}", Style::new().fg(Fixed(color)).bold().paint(text))
}
