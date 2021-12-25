use hafas_rs::requester::hyper::HyperRustlsRequester;
use hafas_rs::profile::db::DbProfile;
use hafas_rs::client::HafasClient;
use hafas_rs::api::journeys::JourneyOptions;

#[tokio::main]
async fn main() {
    let c = HafasClient::new(DbProfile, HyperRustlsRequester::new());
    let from = c.suggestions("Berlin", None).await.unwrap()[0].clone();
    let to = c.suggestions("Kiel", None).await.unwrap()[0].clone();
    println!("{:#?}", &from);
    println!("{:#?}", &to);
    let journeys = c.journeys(from, to, JourneyOptions::default()).await.unwrap();
    println!("{:#?}", journeys);
}
