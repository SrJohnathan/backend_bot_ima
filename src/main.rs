#[macro_use]
extern crate rocket;

use std::env;
use std::path::PathBuf;
use rocket::data::{Limits, ToByteUnit};

mod cofg;
mod domain;
mod webhooks;
mod routes;
mod auth;

#[tokio::main]
async fn main() {



    let routes_api = routes![
        routes::login ,
        webhooks::whats_gupshup::webhook,
        webhooks::whats_gupshup::message
    ];


    let pat = env::current_dir().unwrap();
    let path = pat.as_path().to_str().unwrap();

    let figment = rocket::Config::figment()
        .merge(("port", 8002))
        .merge(("temp_dir", PathBuf::from(format!("{}/imge_cli", path))))
        .merge(("limits", Limits::new().limit("json", 2.mebibytes())));


    rocket::custom(figment)
        .mount("/",
               routes_api,
        ).launch().await.expect("Error rum app");
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_add() {
       // assert_eq!(add(1, 2), 3);
    }

}