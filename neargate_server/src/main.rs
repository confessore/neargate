#[macro_use] extern crate rocket;
use rocket::{get, serde::{json::Json, Serialize}};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Ping<'a> {
    message: &'a str,
 }

#[get("/ping")]
fn ping() -> Json<Ping<'static>> {
    Json(Ping { message:"Pong" })
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, ping])
}