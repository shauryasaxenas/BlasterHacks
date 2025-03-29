use tui::{
    style::{Color, Modifier, Style},
    widgets::{ListState, TableState},
};
use std::time::Duration;
use crate::types::data::Data;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum ActiveBlock {
    Home,
    Assignments,
    Grades,
}

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub enum Screen {
    Default,
    Help,
}

pub struct Route {
    pub screen: Screen,
    pub selected_block: ActiveBlock,
    pub active_block: ActiveBlock,
}

impl Route {
    pub fn default() -> Route {
        Route {
            screen: Screen::Default,
            selected_block: ActiveBlock::Assignments,
            active_block: ActiveBlock::Home,
        }
    }
}

pub struct App {
    pub tick_rate: Duration,
    pub route: Route,
    pub assignments_state: TableState,
    pub data: Data,
}

impl App {
    pub fn new(data: Data, tick_rate: Duration) -> App {
        App {
            tick_rate,
            route: Route::default(),
            assignments_state: TableState::default(),
            data,
        }
    }

    pub fn mv(&mut self, dir: Dir) {
        match dir {
            Dir::Left => match self.route.selected_block {
                ActiveBlock::Grades => self.route.selected_block = ActiveBlock::Assignments,
                _ => (),
            },
            Dir::Right => match self.route.selected_block {
                ActiveBlock::Assignments => self.route.selected_block = ActiveBlock::Grades,
                _ => (),
            },
            _ => (),
        }
    }

    pub fn esc(&mut self) {
        self.route.active_block = ActiveBlock::Home;
    }

    pub fn enter(&mut self) {
        self.route.active_block = self.route.selected_block;
    }

    pub fn get_border_style_from_id(&self, id: ActiveBlock) -> Style {
        let style = Style::default();

        if id == self.route.active_block {
            return style.fg(Color::LightGreen).add_modifier(Modifier::BOLD);
        } else if id == self.route.selected_block {
            return style.fg(Color::LightBlue).add_modifier(Modifier::BOLD);
        } else {
            return style.fg(Color::Gray);
        }
    }

    pub fn get_highlight_style_from_id(&self, id: ActiveBlock) -> Style {
        let style = Style::default().add_modifier(Modifier::BOLD);

        if id == self.route.active_block {
            return style.fg(Color::LightGreen);
        } else if id == self.route.selected_block {
            return style.fg(Color::LightBlue);
        } else {
            return style.fg(Color::White);
        }
    }


    pub fn on_tick(&mut self) {
        ()
    }
}
