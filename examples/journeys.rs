#![feature(backtrace)]

use hafas_client::requester::hyper::HyperRustlsRequester;
use hafas_client::profile::sncf::SncfProfile;
use hafas_client::client::HafasClient;
use hafas_client::api::journeys::JourneysOptions;
use hafas_client::TariffClass;
use std::error::Error;

#[tokio::main]
async fn main() {
    let c = HafasClient::new(SncfProfile, HyperRustlsRequester::new());
    let from = c.locations("Capvern", None).await.unwrap()[0].clone();
    let to = c.locations("Paris Montparnasse", None).await.unwrap()[0].clone();
    eprintln!("{:#?}", &from);
    eprintln!("{:#?}", &to);
    let mut opts = JourneysOptions::default();
    //opts.stopovers = Some(true);
    //opts.polylines = Some(true);
    //opts.tariff_class = Some(TariffClass::First);
    //opts.departure = Some(1643413595);
    match c.journeys(from, to, opts).await {
        Err(e) => {
            if let Some(bt) = e.backtrace() {
                eprintln!("{}", bt);
            }
            eprintln!("{}", e);
        },
        Ok(journeys) => println!("{}", serde_json::to_string(&journeys).unwrap()),
    }
}
