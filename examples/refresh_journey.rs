#![feature(backtrace)]

use hafas_client::requester::hyper::HyperRustlsRequester;
use hafas_client::profile::db::DbProfile;
use hafas_client::client::HafasClient;
use hafas_client::api::refresh_journey::RefreshJourneyOptions;
use hafas_client::TariffClass;
use std::error::Error;

#[tokio::main]
async fn main() {
    let c = HafasClient::new(DbProfile, HyperRustlsRequester::new());
    let mut opts = RefreshJourneyOptions::default();
    let refresh_token = "¶HKI¶T$A=1@O=Augsburg Hbf@L=8000013@a=128@$A=1@O=Ingolstadt Hbf@L=8000183@a=128@$202202022045$202202022149$BRB86714$$1$$$$§T$A=1@O=Ingolstadt Hbf@L=8000183@a=128@$A=1@O=Nürnberg Hbf@L=8000284@a=128@$202202022157$202202022241$RE  4048$$1$$$$§T$A=1@O=Nürnberg Hbf@L=8000284@a=128@$A=1@O=Hof Hbf@L=8002924@a=128@$202202022257$202202030058$RE  3099$$1$$$$§T$A=1@O=Hof Hbf@L=8002924@a=128@$A=1@O=Chemnitz Hbf@L=8010184@a=128@$202202030428$202202030603$RE 74003$$1$$$$§T$A=1@O=Chemnitz Hbf@L=8010184@a=128@$A=1@O=Elsterwerda@L=8010099@a=128@$202202030609$202202030736$RB 74116$$1$$$$§T$A=1@O=Elsterwerda@L=8010099@a=128@$A=1@O=Berlin Hbf (tief)@L=8098160@a=128@$202202030756$202202030932$RE 52272$$1$$$$";
    match c.refresh_journey(refresh_token, opts).await {
        Err(e) => {
            if let Some(bt) = e.backtrace() {
                eprintln!("{}", bt);
            }
            eprintln!("{}", e);
        },
        Ok(journeys) => println!("{}", serde_json::to_string(&journeys).unwrap()),
    }
}
