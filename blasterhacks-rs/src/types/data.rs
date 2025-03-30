use std::error::Error;
use crate::groq;
use crate::types::assignment::Assignment;
use crate::types::grade::Grade;
use crate::types::link::Link;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub assignments: Vec<Assignment>,
    pub grades: Vec<Grade>,
    pub plan: String,
}

impl Data {
    pub async fn from_course_ids(course_ids: Vec<u32>, debug: bool) -> Result<Self, Box<dyn std::error::Error>> {
        if debug {
            println!("Fetching assignments...");
        }
        let mut assignments = crate::queries::assignments::query_assignments(&course_ids).await?;
        if debug {
            println!("Fetched {} assignments", assignments.len());
            println!("Fetching grades...");
        }
        let grades = crate::queries::grades::query_grades(&course_ids).await?;
        if debug {
            println!("Fetched {} grades", grades.len());
            println!("Begin groq analysis...");
        }
        groq_analysis(&mut assignments).await?;
        let plan = groq::get_plan(&assignments).await?;
        if debug {
            println!("Groq analysis complete");
        }

        Ok(Self { assignments, grades, plan })
    }

    pub fn serialize(&self) -> Result<String, Box<dyn Error>> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn serialize_to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let data = self.serialize()?;
        std::fs::write(path, data)?;

        Ok(())
    }

    pub fn deserialize(data: &str) -> Result<Self, Box<dyn Error>> {
        Ok(serde_json::from_str(data)?)
    }

    pub fn deserialize_from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let data = std::fs::read_to_string(path)?;
        Self::deserialize(&data)
    }
}

pub async fn groq_analysis(assignments: &mut Vec<Assignment>) -> Result<(), Box<dyn Error>> {
    for a in assignments {
        a.relevant_links = vec![];
        a.relevant_links.push(Link::new(a.html_url.clone(), "Canvas".to_string()));
        let summary = if let Some(description) = &a.description {
            let links = groq::get_links(&description).await?;
            for link in links {
                a.relevant_links.push(link);
            }
            match groq::get_summary(&description, &a.name).await?.as_str() {
                "No summary" => None,
                s => Some(s.to_string()),
            }
        } else {
            None
        };
        a.summary = summary;
    }

    Ok(())
}
