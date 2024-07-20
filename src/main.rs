extern crate dotenv_codegen;

use rocket::{get, routes};
use rocket::serde::{Serialize, json::Json};
use rocket::http::ContentType;

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
    let runtime = tokio::spawn(async move {
        let _ = rocket::build().mount("/", routes![index_route])
            .launch().await;
    });

    runtime.await.unwrap();
}