#[macro_use] extern crate rocket;

use rocket::serde::{json::{self, Json}, Deserialize, Serialize};
use reqwest::{Response, Client};
use std::env;

#[cfg(test)] mod tests;


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

fn add(arg1: i64, arg2: i64) -> i64 {
    arg1 + arg2
}

fn sub(arg1: i64, arg2: i64) -> i64 {
    arg1 - arg2
}

fn get_service2_ip() -> String {
    let v = match env::var("SERVICE2_IP") {
        Ok(val) => {
            val
        }
        Err(_) => {
            "127.0.0.1:8002".to_string()
        }
    };
    println!("SERVICE2_IP = {}", v);
    v
}

async fn mul(arg1: i64, arg2: i64) -> i64  {
    let client = Client::new();
    let url = format!("http://{}/math/mul", get_service2_ip());
    println!("{} {} {}", url, arg1, arg2);
    let response = client.post(url)
        .body(format!("{{\"arg1\": {}, \"arg2\": {}}}", arg1, arg2))
        .send()
        .await.unwrap();

    if response.status().is_success() {
        let result_data = response.text().await.unwrap();
        let parsed: ResultValue = json::from_str(&result_data.to_string()).unwrap();
        parsed.result
    } else {
        666
    }
}

async fn div(arg1: i64, arg2: i64) -> i64  {
    let client = Client::new();
    let url = format!("http://{}/math/div", get_service2_ip());
    let response = client.post(url)
        .body(format!("{{\"arg1\": {}, \"arg2\": {}}}", arg1, arg2))
        .send()
        .await.unwrap();

    if response.status().is_success() {
        let result_data = response.text().await.unwrap();
        let parsed: ResultValue = json::from_str(&result_data.to_string()).unwrap();
        parsed.result
    } else {
        666
    }
}

#[post("/<op>",  data = "<args>")] 
async fn math(op: & str, args: Json<Args>) -> Json<ResultValue> {
    println!("{} {} {}", op, args.arg1, args.arg2);
    match op {
        "add" => Json(ResultValue { result: add(args.arg1, args.arg2) }),
        "sub" => Json(ResultValue { result: sub(args.arg1, args.arg2) }),
        "mul" => Json(ResultValue { result: mul(args.arg1, args.arg2).await}),
        "div" => Json(ResultValue { result: div(args.arg1, args.arg2).await}),
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
