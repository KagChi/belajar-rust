#[macro_use]
extern crate dotenv_codegen;

use std::sync::Arc;

use rocket::{config::Config, get, routes, futures::StreamExt};
use rocket::serde::{Serialize, json::Json};
use rocket::http::{ContentType};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{Cluster, Intents, Event};
use anyhow::Result;
use once_cell::sync::Lazy;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct IndexMessage<'a> {
    message: &'a str
}

static CACHE: Lazy<InMemoryCache> = Lazy::new(|| { 
        InMemoryCache::builder()
            .resource_types(ResourceType::PRESENCE | ResourceType::USER | ResourceType::GUILD)
            .build()
    });

#[get("/")]
fn index_route() -> (ContentType, Json<IndexMessage<'static>>)  {
    (ContentType::JSON, 
        Json(IndexMessage {
                message: "Hello, world!"
            }
        )
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    tokio::spawn(async move {
        let port = Result::from(dotenv!("PORT").parse::<u16>())
            .unwrap_or("3000".parse::<u16>().unwrap());

        let config = Config::figment()
            .merge(("port", port)
        );
        let rocket_instance = rocket::custom(config)
            .mount("/", routes![index_route])
            .launch().await;
        match rocket_instance {
            Ok(_) => println!("Rocket instance launched"),
            Err(e) => println!("Rocket instance failed to launch: {}", e)
        }
    });

    let (cluster, mut events) = Cluster::new(dotenv!("DISCORD_TOKEN").to_owned(), Intents::GUILD_PRESENCES | Intents::GUILDS).await?;
    let cluster = Arc::new(cluster);

    let cluster_spawn = Arc::clone(&cluster);

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    while let Some((shard_id, event)) = events.next().await {
       CACHE.update(&event);

       match event {
           Event::PresenceUpdate(presence) => println!("Presence update: {:?}", presence),
           Event::ShardConnected(_) => println!("Connected on shard {shard_id}"),
            _ => {}
       }
    }


    Ok(())
}