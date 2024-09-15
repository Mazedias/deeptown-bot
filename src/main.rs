mod db;
mod bot;
mod logic;

use dotenv::dotenv;
use std::env;
use tokio;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let discord_token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");

    let db_pool = db::establish_connection(&database_url);

    bot::start_bot(discord_token, db_pool).await;
}
