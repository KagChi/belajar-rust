#[macro_use]
extern crate dotenv_codegen;

use rocket::{config::{Config}, get, routes};
use tokio::time::{Instant, interval_at};
use rocket::serde::{Serialize, json::Json};
use rocket::http::{Status, ContentType};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct IndexMessage<'a> {
    message: &'a str
}

#[get("/")]
fn index() -> (Status, (ContentType, Json<IndexMessage<'static>>))  {
    (Status::Ok, 
        (ContentType::JSON, 
            Json(IndexMessage {
                message: "Hello, world!"
                }
            )
        )
    )
}

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        let port = Result::from(dotenv!("PORT").parse::<u16>())
            .unwrap_or("3000".parse::<u16>().unwrap());

        let config = Config::figment()
            .merge(("port", port)
        );
        let rocket_instance = rocket::custom(config).mount("/", routes![index]).launch().await;
        match rocket_instance {
            Ok(_) => println!("Rocket instance launched"),
            Err(e) => println!("Rocket instance failed to launch: {}", e)
        }
    });

    let start = Instant::now();
    let mut interval = interval_at(start, tokio::time::Duration::from_secs(5));

    loop {
        interval.tick().await;
    }

}