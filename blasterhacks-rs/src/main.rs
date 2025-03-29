use std::error::Error;
use tokio;

mod assignment;
mod grade;
mod queries;
mod groq;
mod data;

use assignment::Assignment;
use queries::assignments::query_assignments;
use queries::grades::query_grades;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let course_ids = vec![72125, 71983, 72567, 71447, 72767];

    let data = data::Data::from_course_ids(course_ids).await?;
    data.serialize_to_file("data.json")?;

    Ok(())
}
