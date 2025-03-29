use std::error::Error;
use tokio;

mod types;
mod queries;
mod groq;
mod ui;

use types::data::Data;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let course_ids = vec![72125, 71983, 72567, 71447, 72767];
    // let data = Data::from_course_ids(course_ids).await?;

    let data = Data::deserialize_from_file("data.json")?;
    data.serialize_to_file("data.json")?;
    let res = ui::run(data);
    match res {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
