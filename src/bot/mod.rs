mod commands;

use poise::{ serenity_prelude as serenity, Command};
use diesel::r2d2::{self, ConnectionManager};
use diesel::pg::PgConnection;

pub struct Data {
    db_pool: DbPool,
}

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
type Error = Box<dyn std::error::Error + Send + Sync>;

pub async fn start_bot(token: String, db_connection: DbPool) {
    let intents = serenity::GatewayIntents::non_privileged();

    let commands: Vec<Command<Data, Error>> = vec![];

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
    
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}