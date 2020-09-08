mod commands;
mod events;
pub mod utils;

use crate::config::Config;
use crate::database;

use std::sync::Arc;
use tokio::sync::RwLock;

use events::Handler;
use log::warn;
use serenity::{framework::standard::StandardFramework, prelude::TypeMapKey, Client};

use crate::database::DataBase;

impl TypeMapKey for Config {
    type Value = Arc<RwLock<Config>>;
}

pub async fn start(config: Config) {
    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefix(&config.prefix);
            c.allow_dm(true);
            c.case_insensitivity(true);
            return c;
        })
        .group(&commands::COMMANDS_GROUP);

    let mut client = Client::new(&config.token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Failed to create a new client");

    let db_client = database::connect(&config.db_uri).await;

    {
        let mut data = client.data.write().await;
        data.insert::<Config>(Arc::new(RwLock::new(config)));
        data.insert::<DataBase>(db_client);
    }

    if let Err(e) = client.start().await {
        warn!("Failed to login, is the token correct?\n{}", e);
    }
}
