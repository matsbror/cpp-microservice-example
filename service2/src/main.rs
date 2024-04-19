#[macro_use] extern crate rocket;

use rocket::serde::{Deserialize, Serialize, json::Json, json};

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
#[serde(crate = "rocket::serde")]
struct Args {
    arg1: i64,
    arg2: i64,
}

fn mul(arg1: i64, arg2: i64) -> i64 {
    arg1 * arg2
}

fn div(arg1: i64, arg2: i64) -> i64 {
    arg1 / arg2
}

#[post("/<op>",  data = "<args>")] 
fn math(op: & str, args: Json<Args>) -> Json<ResultValue> {
    print!("{} {} {}", op, args.arg1, args.arg2);
    match op {
        "mul" => Json(ResultValue { result: mul(args.arg1, args.arg2) }),
        "div" => Json(ResultValue { result: div(args.arg1, args.arg2) }),
        _ => Json(ResultValue { result: 0 }),
    }
}

// Try visiting:
//   http://127.0.0.1:8000/usage
#[get("/", format = "json")]
fn usage() -> Json<ResultMessage> {
    Json(ResultMessage { message: format!("This is how you use this service!") })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![usage])
        .mount("/math", routes![math])
        
}
