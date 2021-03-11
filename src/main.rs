#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::Request;
use rocket::catchers;
use rocket_contrib::serve::StaticFiles;

#[catch(400)]
fn bad_request() -> &'static str {
    "Check your connection and settings, clear your cache, open a different browser, and retry."
}

#[catch(401)]
fn auth_required() -> &'static str {
    "You are trying to access a web page that requires a password."
}
#[catch(403)]
fn forbidden() -> &'static str {
    "Usually, getting this error means you have entered a URL or clicked a link that goes to a page that has been set up with access permissions, meaning you have to have an account or some other type of authorization in order to access the page."
}
#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[catch(405)]
fn method_not_allowed() -> &'static str {
    "Please refresh and try again"
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
        .register(catchers![bad_request, auth_required, forbidden, method_not_allowed, not_found, internal_error, service_unavailable])
        .mount("/", StaticFiles::from("static"))
}

fn main() {
    rocket().launch();
}