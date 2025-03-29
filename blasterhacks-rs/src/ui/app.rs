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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Widget {
    Assignments,
    Links,
}

pub struct App {
    pub tick_rate: Duration,
    pub assignments_state: TableState,
    pub links_state: ListState,
    pub data: Data,
    pub active_widget: Widget,
}

impl App {
    pub fn new(data: Data, tick_rate: Duration) -> App {
        App {
            tick_rate,
            assignments_state: TableState::default(),
            links_state: ListState::default(),
            data,
            active_widget: Widget::Assignments,
        }
    }

    pub fn open(&self) {
        match self.active_widget {
            Widget::Assignments => {
                if let Some(i) = self.assignments_state.selected() {
                    let assignment = &self.data.assignments[i];
                    open::that(&assignment.html_url).unwrap();
                }
            },
            Widget::Links => {
                if let Some(i) = self.links_state.selected() {
                    let assignment_index = self.assignments_state.selected().unwrap_or(0);
                    let link = &self.data.assignments[assignment_index].relevant_links[i];
                    open::that(&link.url).unwrap();
                }
            }
        }
    }

    pub fn enter(&mut self) {
        match self.active_widget {
            Widget::Assignments => {
                if let Some(i) = self.assignments_state.selected() {
                    let assignment = &self.data.assignments[i];
                    if assignment.relevant_links.len() > 0 {
                        self.links_state.select(Some(0));
                        self.active_widget = Widget::Links;
                    }
                }
            },
            _ => (),
        }
    }

    pub fn esc(&mut self) {
        match self.active_widget {
            Widget::Assignments => (),
            Widget::Links => {
                self.links_state.select(None);
                self.active_widget = Widget::Assignments;
            }
        }
    }

    pub fn mv(&mut self, dir: Dir) {
        match self.active_widget {
            Widget::Assignments => match dir {
                Dir::Down => self.next_assignment(),
                Dir::Up => self.prev_assignment(),
            }
            Widget::Links => match dir {
                Dir::Down => self.next_link(),
                Dir::Up => self.prev_link(),
            }
        }
        
    }

    pub fn next_link(&mut self) {
        if let Some(selected) = self.links_state.selected() {
            let i = self.assignments_state.selected().unwrap_or(0);
            let next = if selected >= self.data.assignments[i].relevant_links.len() - 1 {
                0
            } else {
                selected + 1
            };
            self.links_state.select(Some(next));
        } else if self.data.assignments.len() > 0 {
            self.links_state.select(Some(0));
        }
    }

    pub fn prev_link(&mut self) {
        if let Some(selected) = self.links_state.selected() {
            let i = self.assignments_state.selected().unwrap_or(0);
            let prev = if selected == 0 {
                self.data.assignments[i].relevant_links.len() - 1
            } else {
                selected - 1
            };
            self.links_state.select(Some(prev));
        } else {
            self.links_state.select(Some(0));
        }
    }

    pub fn next_assignment(&mut self) {
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

    pub fn prev_assignment(&mut self) {
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
