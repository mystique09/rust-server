use mongodb::{Client, options::ClientOptions};

pub async fn setup_db() {
  let mut options = ClientOptions::parse("mongodb://localhost:27017").await?;
  
  options.app_name = Some("My Database".to_string());
  let client = Client::with_options(options)?;
  let db = client.database("codegram");
  db
}