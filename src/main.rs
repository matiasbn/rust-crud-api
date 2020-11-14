#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::json::Json;
use serde::Deserialize;

#[macro_use]
extern crate rocket;

#[derive(Deserialize)]
struct Task {
    complete: bool,
    description: String,
}

#[post("/todo", data = "<task>")]
fn new_post(task: Json<Task>) -> String {
    print!("{}", task.complete);
    format!("{}", task.description)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: String, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, hello, new_post])
        .launch();
}
