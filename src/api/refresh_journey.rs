use crate::{Result, Profile, Requester, Client, TariffClass};
use crate::client::HafasClient;
use ijson::ijson;
use crate::parse::journeys_response::parse_journeys_response;
use crate::parse::journeys_response::HafasJourneysResponse;
use serde::Serialize;
use serde::Deserialize;
use crate::Journey;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct RefreshJourneyOptions {
    pub stopovers: Option<bool>,
    pub polylines: Option<bool>,
    pub tickets: Option<bool>,
    pub tariff_class: Option<TariffClass>,
}

pub type RefreshJourneyResponse = Journey;

impl<P: Profile + Sync + Send, R: Requester + Sync + Send> HafasClient<P, R> {
    pub async fn refresh_journey(
        &self,
        refresh_token: &str,
        options: RefreshJourneyOptions,
    ) -> Result<RefreshJourneyResponse> {
        let tariff_class = options.tariff_class.unwrap_or(TariffClass::Second);

        let data: HafasJourneysResponse = self.request(ijson!({
            "cfg": {},
            "meth": "Reconstruction",
            "req": {
                    "ctxRecon": refresh_token,
                    "getIST": true,
                    "getPasslist": options.stopovers.unwrap_or(false),
                    "getPolyline": options.polylines.unwrap_or(false),
                    "getTariff": options.tickets.unwrap_or(false),
            }
        })).await?;

        let mut journeys = parse_journeys_response(data, tariff_class)?;
        Ok(journeys.journeys.remove(0))
    }
}
