use hafas_client::requester::hyper::HyperRustlsRequester;
use hafas_client::profile::db::DbProfile;
use hafas_client::client::HafasClient;
use hafas_client::api::journeys::JourneyOptions;
use hafas_client::TariffClass;

#[tokio::main]
async fn main() {
    let c = HafasClient::new(DbProfile, HyperRustlsRequester::new());
    let from = c.suggestions("Berlin Hbf", None).await.unwrap()[0].clone();
    let to = c.suggestions("Hannover Hbf", None).await.unwrap()[0].clone();
    //println!("{:#?}", &from);
    //println!("{:#?}", &to);
    let mut opts = JourneyOptions::default();
    opts.stopovers = Some(true);
    //opts.polylines = Some(true);
    //opts.tariff_class = Some(TariffClass::First);
    //opts.departure = Some(1643413595);
    let journeys = c.journeys(from, to, opts).await.unwrap();
    println!("{}", serde_json::to_string(&journeys).unwrap());
}
