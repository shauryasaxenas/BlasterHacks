use graphql_client::{GraphQLQuery, Response};
use reqwest;
use std::error::Error;
use tokio;
use assignment::{Assignment};

mod assignment;

// Define the GraphQL API endpoint
const API_URL: &str = "https://elearning.mines.edu/api/graphql";
const API_TOKEN: &str = "9802~VLrYDYHhHtavZQGLZXzRmWNDaamKCrF8R7AR2VkxYUUWMfNXZNxFX97ZatPZZzAe";

// Define the GraphQL query
type URL = String;
type DateTime = String;
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",  // Update this with the correct schema path
    query_path = "src/get_assignments.graphql", // Query is stored in a separate file
    response_derives = "Debug"
)]
struct GetAssignments;

async fn perform_queries(course_ids: Vec<u32>) -> Result<Vec<get_assignments::ResponseData>, Box<dyn Error>> {
    let mut responses: Vec<get_assignments::ResponseData> = vec![];
    for course_id in course_ids {
        let variables = get_assignments::Variables { course_id: course_id.to_string() };
        let response = perform_query(variables).await?;
        responses.push(response);
    }

    Ok(responses)
}

async fn perform_query(variables: get_assignments::Variables) -> Result<get_assignments::ResponseData, Box<dyn Error>> {
    // this is the important line
    let request_body = GetAssignments::build_query(variables);

    let client = reqwest::Client::new();
    let res = client
        .post(API_URL)
        .bearer_auth(API_TOKEN)
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<get_assignments::ResponseData> = res.json().await?;

    match response_body.data {
        Some(data) => {
            Ok(data)
        }
        None => {
            Err("No data found".into())
        }
    }
}

fn parse_assignments(responses: Vec<get_assignments::ResponseData>) -> Result<Vec<Assignment>, Box<dyn Error>> {
    let mut assignments: Vec<Assignment> = vec![];
    for response in responses {
        if let Some(course) = response.course {
            // Iterate over assignments
            for a in course.assignments_connection.unwrap().nodes.unwrap() {
                let a = a.unwrap();
                let assignment: Assignment = Assignment::new(a.name.clone().unwrap(), course.course_nickname.clone(), a.description, a.html_url.clone().unwrap(),  a.due_at, course.name.clone())?;
                if let Some(due) = assignment.date {
                    // If assignment due within 14 days, add to list
                    let now = chrono::Utc::now();
                    if due > now && due < now + chrono::Duration::days(14) {
                        assignments.push(assignment);
                    }
                }
            }
        } else {
            eprintln!("No course data found");
        }
    }

    return Ok(assignments);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create an empty Variables struct
    let variables = get_assignments::Variables { course_id: "72125".to_string() };
    

    let course_ids = vec![72125, 71983, 72567, 71447, 72767];
    let responses = perform_queries(course_ids).await?;

    // Parse the response
    let assignments = parse_assignments(responses)?;
    for a in assignments {
        println!("{}", a);
    }

    Ok(())
}
