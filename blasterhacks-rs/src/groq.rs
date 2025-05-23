use std::error::Error;
use crate::types::assignment::Assignment;
use crate::types::link::Link;
use reqwest::Client;
use serde_json::json;


pub async fn get_response(query: String) -> Result<String, Box<dyn Error>> {
    let api_key = std::env::var("GROQ_API_KEY")?;
    let client = Client::new();
    let url = "https://api.groq.com/openai/v1/chat/completions";

    let body = json!({
        "messages": [{
            "role": "user",
            "content": query
        }],
        "model": "llama-3.3-70b-versatile"
    });

    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let response_json: serde_json::Value = response.json().await?;
    if let Some(content) = response_json["choices"][0]["message"]["content"].as_str() {
        return Ok(content.to_string());
    } else {
        eprintln!("{:?}", response_json);
        return Err(format!("No content found for query: {}", query).into());
    }

}

pub async fn get_summary(description: &String, name: &String) -> Result<(bool, String), Box<dyn Error>> {
    let mut has_description: bool = true;
    let mut response = get_response("You will recieve HTML with that describes a certain university assignment. Summarize it and return a signle paragraph. If there is no relevant information simply reply with the words \"No summary\"".to_string() + &description).await?;
    if response == "No summary" {
        has_description = false;
        response = get_response("You will recieve the name of a university assignment. Summarize what the assignment is about and return a single paragraph.".to_string() + &name).await?;
    }
    return Ok((has_description, response));
}

pub async fn get_links(description: &String) -> Result<Vec<Link>, Box<dyn Error>> {
    let response = match get_response("You will recieve HTML with that describes a certain university assignment. Extract all the links from the text and return them one line at a time with the format title,link. Ignore .js and .css files. If there are no links respond with the word \"None\"".to_string() + &description).await {
        Ok(response) => response,
        Err(_) => "None".to_string()
    };
    if response == "None" {
        return Ok(vec![]);
    }
    let links: Vec<Link> = response.lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                Some(Link::new(parts[1].to_string(), parts[0].to_string()))
            } else {
                None
            }
        })
        .collect();
    Ok(links)
}

pub async fn get_plan(assignments: &Vec<Assignment>) -> Result<String, Box<dyn Error>> {
    let mut message = String::from("Create a study plan for the following assignments (short paragraph):\n");
    for assignment in assignments {
        if let Some(date) = &assignment.date {
            message.push_str(&format!("{}: {}\n", date.format("%A %d, %H:%M"), assignment.name));
        }
    }
    let response = get_response(message).await?;
    Ok(response)
}
