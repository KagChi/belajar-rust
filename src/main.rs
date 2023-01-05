#[macro_use]
extern crate dotenv_codegen;

use rocket::{config::{Config}, get, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    tokio::spawn(async {
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
}