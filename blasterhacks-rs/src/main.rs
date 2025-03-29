use graphql_client::{GraphQLQuery, Response};
use reqwest;
use std::error::Error;
use serde::Serialize;
use tokio;

// Define the GraphQL API endpoint
const API_URL: &str = "https://elearning.mines.edu/api/graphql";
const API_TOKEN: &str = "9802~VLrYDYHhHtavZQGLZXzRmWNDaamKCrF8R7AR2VkxYUUWMfNXZNxFX97ZatPZZzAe";

// Define the GraphQL query
#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.json",  // Update this with the correct schema path
    query_path = "src/get_courses.graphql", // Query is stored in a separate file
    response_derives = "Debug"
)]
struct GetCourses;

async fn perform_my_query(variables: get_courses::Variables) -> Result<(), Box<dyn Error>> {

    // this is the important line
    let request_body = GetCourses::build_query(variables);

    let client = reqwest::Client::new();
    let res = client
        .post(API_URL)
        .bearer_auth(API_TOKEN)
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<get_courses::ResponseData> = res.json().await?;

    response_body.data.unwrap().all_courses.unwrap().iter().for_each(|course| {
        println!("Course: {:#?}", course.name);
    });
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create an empty Variables struct
    let variables = get_courses::Variables {};
    
    // Call the async function with the correct variables
    perform_my_query(variables).await?;
    
    Ok(())
}
