#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::Request;
use rocket::catchers;
use rocket_contrib::serve::StaticFiles;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[catch(500)]
fn internal_error() -> &'static str {
    "Whoops! Looks like we messed up."
}

#[catch(503)]
fn service_unavailable() -> &'static str {
    "When you get a 503 error, it could mean the server is undergoing maintenance, or is overloaded with requests."
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .register(catchers![not_found, internal_error, service_unavailable])
        .mount("/", StaticFiles::from("static"))
}

fn main() {
    rocket().launch();
}