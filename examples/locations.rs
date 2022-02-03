#![feature(backtrace)]

use hafas_client::requester::hyper::HyperRustlsRequester;
use hafas_client::profile::db::DbProfile;
use hafas_client::client::HafasClient;
use std::error::Error;

#[tokio::main]
async fn main() {
    let c = HafasClient::new(DbProfile, HyperRustlsRequester::new());
    match c.locations("Berlin", Some(10)).await {
        Err(e) => {
            if let Some(bt) = e.backtrace() {
                eprintln!("{}", bt);
            }
            eprintln!("{}", e);
        },
        Ok(journeys) => println!("{}", serde_json::to_string(&journeys).unwrap()),
    }
}
