#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::kindle::Kindle;
use rocket::http::Status;
use rocket::response;
use rocket::{http::ContentType, response::Redirect};
use serde_json::json;
use std::io::Cursor;

mod constant;
mod kindle;

#[get("/")]
fn index() -> String {
    let mut instruction =
        String::from("Hello There these are Serial Numbers of Kindles in the API\n");
    let scrape = Kindle::scrape_ota();
    for i in scrape.into_iter() {
        instruction = format!(
            "{}{}. {} - {}\n",
            instruction,
            i.sno(),
            i.name(),
            i.version()
        );
    }
    instruction
}
#[get("/json")]
fn json<'r>() -> response::Result<'r> {
    let json = serde_json::to_string_pretty(&json!(Kindle::scrape_ota())).unwrap();
    response::Response::build()
        .header(ContentType::JSON)
        .sized_body(Cursor::new(json))
        .ok()
}
#[get("/kindle/<kindle_no>")]
fn get_kindle<'r>(kindle_no: i32) -> response::Result<'r> {
    let vector = Kindle::scrape_ota();
    let kindle = vector.into_iter().nth(kindle_no as usize);
    if kindle.is_none() {
        return Err(Status::NotFound);
    }
    let json = serde_json::to_string_pretty(&json!(kindle)).unwrap();
    response::Response::build()
        .header(ContentType::JSON)
        .sized_body(Cursor::new(json))
        .ok()
}
#[get("/kindle/<kindle_no>/download")]
fn download_latest(kindle_no: i32) -> Option<Redirect> {
    let vector = kindle::Kindle::scrape_ota();
    let kindle = vector.into_iter().nth(kindle_no as usize)?;
    let dw_link = kindle.dw_link();
    Some(Redirect::to(dw_link))
}
fn main() {
    rocket::ignite()
        .mount("/", routes![index, json, get_kindle, download_latest])
        .launch();
}
