use std::env;

use log::Level;
use log::{error, info};

use dotenv::dotenv;
use reddit_bot::bot::init_bot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().unwrap();
    simple_logger::init_with_level(Level::Info).expect("Failed to init logger");
    let token = env::var("TG_TOKEN").expect("Missing TG_TOKEN env var");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    init_bot(&token, &database_url).await;

    Ok(())
}
