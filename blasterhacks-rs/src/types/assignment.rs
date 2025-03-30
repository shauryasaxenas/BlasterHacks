use crate::types::link::Link;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Assignment {
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub date: Option<DateTime<FixedOffset>>,
    pub course: String,
    pub summary: Option<String>,
    pub relevant_links: Vec<Link>,
    pub has_description: bool,
}

impl Assignment {
    pub fn new(name: String, nickname: Option<String>, description: Option<String>, html_url: String, datestring: Option<String>, course: String) -> Result<Self, Box<dyn std::error::Error>> {
        let course = if let Some(nickname) = nickname {
            nickname
        } else {
            course
        };

        let date = if let Some(datestring) = datestring {
            Some(DateTime::parse_from_rfc3339(&datestring)?)
        } else {
            None
        };

        Ok(Self {
            name,
            description,
            html_url,
            date,
            course,
            summary: None,
            relevant_links: vec![],
            has_description: false,
        })
    }
}

impl std::fmt::Display for Assignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(date) = self.date {
            write!(f, "{} - {} ({})", self.course, self.name, date.format("%A %d"))
        } else {
            write!(f, "{} - {} (No due date)", self.course, self.name)
        }
    }
}
