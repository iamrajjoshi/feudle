
#[macro_use] extern crate rocket;
// use crate::client::events;
use crate::client::answer;
use crate::client::events;
use crate::client::ready;
use crate::client::guess;
use crate::client::end;
mod client;
mod feudle;
// use rocket_contrib::json::Json;
// use rocket::response::content;
// use rocket::{Config};
use rocket::fs::{relative, FileServer};
use std::thread;
// use rocket::{State, Shutdown};
// use rocket::form::Form;
// use rocket::response::stream::{EventStream, Event};
// use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
// use rocket::tokio::select;
// use rocket::tokio::time::{self, Duration};


#[launch]
fn rocket() -> _ {
    thread::spawn(move || {    
        client::client();});
    // let config = 
    // .address("1.2.3.4".into())
    // .port(9234);

    rocket::build()
        .mount("/", routes![guess, end, ready, events, answer])
        .mount("/", FileServer::from(relative!("temp"))) // to host the html file. 
    
}