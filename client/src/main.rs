
#[macro_use] extern crate rocket;
use crate::client::answer;
use crate::client::events;
use crate::client::ready;
use crate::client::guess;
use crate::client::end;
mod client;
mod feudle;
use rocket::fs::{relative, FileServer};
use std::thread;


#[launch]
fn rocket() -> _ {
    thread::spawn(move || {client::client();});
    rocket::build()
        .mount("/", routes![guess, end, ready, events, answer])
        .mount("/", FileServer::from(relative!("temp"))) // to host the html file. 
}