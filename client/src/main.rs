
#[macro_use] extern crate rocket;

mod client;
mod feudle;
// use rocket_contrib::json::Json;
// use rocket::response::content;
// use rocket::{Config};
use rocket::fs::{relative, FileServer};
use std::thread;




#[get("/guess/<word>")]
fn foo(word : &str) -> String {
    "alert".to_string()
}



#[launch]
fn rocket() -> _ {
    thread::spawn(move || { 
        
        client::client();});
    // let config = 
    // .address("1.2.3.4".into())
    // .port(9234);

    rocket::build()
        .mount("/", routes![foo])
        .mount("/", FileServer::from(relative!("temp"))) // to host the html file. 
    
}