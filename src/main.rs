#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use crate::kindle::Kindle;
use rocket::http::Status;
use rocket::{http::ContentType, response};
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
#[get("/kindle/<kindle_no>/download?<version>")]
fn download_latest<'r>(kindle_no: i32, version: Option<String>) -> response::Result<'r> {
    let vector = kindle::Kindle::scrape_ota();
    let kindle = vector.into_iter().nth(kindle_no as usize);
    if kindle.is_none() {
        return Err(Status::NotFound);
    }
    if version.is_some()
        && !matches!(
            kindle.as_ref().unwrap().version().cmp(&version.unwrap()),
            std::cmp::Ordering::Greater
        )
    {
        return response::Response::build()
            .header(ContentType::Plain)
            .status(Status::BadRequest)
            .sized_body(Cursor::new(constant::UPDATED))
            .ok();
    }
    let dw_link = kindle.unwrap().dw_link();
    response::Response::build()
        .status(Status::SeeOther)
        .raw_header("Location", dw_link)
        .ok()
}
fn main() {
    rocket::ignite()
        .mount("/", routes![index, json, get_kindle, download_latest])
        .launch();
}
