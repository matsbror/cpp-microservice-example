#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::serde::{Deserialize, Serialize};
use rocket_contrib::json::Json;


#[cfg(test)] mod tests;

#[derive(FromForm)]
struct Options<'r> {
    emoji: bool,
    name: Option<&'r str>,
}



#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct InputData {
    message: String,
}


#[derive(Deserialize,Serialize)]
#[serde(crate = "rocket::serde")]
struct ResultMessage {
    message: String,
}

#[derive(Serialize,Deserialize)]
#[serde(crate = "rocket::serde")]
struct ResultValue {
    result: i64,
}

#[derive(Deserialize,Serialize)]
struct Args {
    arg1: i64,
    arg2: i64,
}

fn add(arg1: i64, arg2: i64) -> i64 {
    arg1 + arg2
}

fn sub(arg1: i64, arg2: i64) -> i64 {
    arg1 - arg2
}

#[post("/math/<op>", format = "json", data = "<args>")]
fn math(op: & str, args: Json<Args>) -> Json<ResultValue> {
    match op {
        "add" => Json(ResultValue { result: add(args.arg1, args.arg2) }),
        "sub" => Json(ResultValue { result: sub(args.arg1, args.arg2) }),
    }
}

// Try visiting:
//   http://127.0.0.1:8000/usage
#[get("/usage", format = "json")]
fn usage() -> Json<ResultMessage> {
    Json(ResultMessage { message: format!("This is how you use this service!") })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![usage])
        .mount("/math", routes![math])
        
}
