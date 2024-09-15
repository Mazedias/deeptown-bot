mod commands;

use poise::{ serenity_prelude as serenity, Command};
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;

/// Data that is accessible in all command invocations
pub struct Data {
    /// Pool of database connections
    db_pool: DbPool,
}

/// Database connection pool type
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
/// Error type for discord bot
type Error = Box<dyn std::error::Error + Send + Sync>;


/// Startup logic of the discord bot
/// 
/// # Arguments
/// 
/// * `token` - Discord token
/// * `db_connection` - Pool of database connections
pub async fn start_bot(token: String, db_connection: DbPool) {
    let intents = serenity::GatewayIntents::non_privileged();

    // Array to registrate functions as discord commands
    let commands: Vec<Command<Data, Error>> = vec![];

    // Create client setup
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    db_pool: db_connection,
                })
            })
        })
        .build();
        
    // Create client
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    
    // Start client
    client.unwrap().start().await.unwrap();
}