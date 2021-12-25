use hafas_rs::requester::hyper::HyperRustlsRequester;
use hafas_rs::profile::db::DbProfile;
use hafas_rs::client::HafasClient;

#[tokio::main]
async fn main() {
    let c = HafasClient::new(DbProfile, HyperRustlsRequester::new());
    println!("{:#?}", c.suggestions("Berlin", None).await.unwrap());
}
