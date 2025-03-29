use tui::{
    style::{Color, Modifier, Style},
    widgets::{ListState, TableState},
};
use std::time::Duration;
use crate::types::data::Data;

pub enum Dir {
    Up,
    Down,
}

pub struct App {
    pub tick_rate: Duration,
    pub assignments_state: TableState,
    pub links_state: ListState,
    pub data: Data,
}

impl App {
    pub fn new(data: Data, tick_rate: Duration) -> App {
        App {
            tick_rate,
            assignments_state: TableState::default(),
            links_state: ListState::default(),
            data,
        }
    }

    pub fn mv(&mut self, dir: Dir) {
        match dir {
            Dir::Down => self.next(),
            Dir::Up => self.prev(),
        }
    }

    pub fn next(&mut self) {
        if let Some(selected) = self.assignments_state.selected() {
            let next = if selected >= self.data.assignments.len() - 1 {
                0
            } else {
                selected + 1
            };
            self.assignments_state.select(Some(next));
        } else if self.data.assignments.len() > 0 {
            self.assignments_state.select(Some(0));
        }
    }

    pub fn prev(&mut self) {
        if let Some(selected) = self.assignments_state.selected() {
            let prev = if selected == 0 {
                self.data.assignments.len() - 1
            } else {
                selected - 1
            };
            self.assignments_state.select(Some(prev));
        } else if self.data.assignments.len() > 0 {
            self.assignments_state.select(Some(0));
        }
    }

    pub fn on_tick(&mut self) {
        ()
    }
}
