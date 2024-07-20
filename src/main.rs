use std::env;
use std::net::IpAddr;

use rocket::{get, routes, Config};
use rocket::serde::{Serialize, json::Json};
use rocket::http::ContentType;
use dotenv::dotenv;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct IndexMessage<'a> {
    message: &'a str
}

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
async fn main() {
    dotenv().ok();

    tokio::spawn(async move {
        let port: u16 = env::var("PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3000);

        let host: IpAddr = env::var("HOST")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or("0.0.0.0".parse().expect("Failed to parse default host"));
        
        let config = Config::figment()
            .merge(("port", port))
            .merge(("host", host));

        let _ = rocket::custom(config)
            .mount("/", routes![index_route])
                .launch().await;
    }).await.unwrap();
}