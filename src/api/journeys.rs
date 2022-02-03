use chrono::offset::Utc;
use chrono::NaiveDateTime;
use crate::{Place, Accessibility, Products, Result, Profile, Requester, Client, Error, TariffClass};
use crate::client::HafasClient;
use crate::format::ToHafas;
use ijson::ijson;
use crate::parse::journeys_response::parse_journeys_response;
use serde::Serialize;
use serde::Deserialize;
use crate::Journey;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct JourneysOptions {
    //pub via: Option<Place>,
    pub earlier_ref: Option<String>,
    pub later_ref: Option<String>,
    pub results: Option<u64>,
    pub stopovers: Option<bool>,
    pub polylines: Option<bool>,
    //pub remarks: Option<bool>,
    //pub bike_friendly: Option<bool>,
    pub tickets: Option<bool>,
    pub start_with_walking: Option<bool>,
    //pub scheduled_days: Option<bool>,
    pub accessibility: Option<Accessibility>,
    pub transfers: Option<i64>,
    pub transfer_time: Option<u64>,
    pub arrival: Option<i64>,
    pub departure: Option<i64>,
    pub products: Option<Products>,
    pub tariff_class: Option<TariffClass>,
}

#[derive(Debug, Serialize)]
pub struct JourneysResponse {
    pub earlier_ref: Option<String>,
    pub later_ref: Option<String>,
    pub journeys: Vec<Journey>,
}

impl<P: Profile + Sync + Send, R: Requester + Sync + Send> HafasClient<P, R> {
    pub async fn journeys(
        &self,
        from: Place,
        to: Place,
        options: JourneysOptions,
    ) -> Result<JourneysResponse> {
        let (when, is_departure) = match (options.departure, options.arrival) {
            (Some(_), Some(_)) => {
                Err(Error::InvalidInput("departure and arrival are mutually exclusive".to_string()))?
            },
            (Some(departure), None) => (NaiveDateTime::from_timestamp(departure, 0), true),
            (None, Some(arrival)) => (NaiveDateTime::from_timestamp(arrival, 0), false),
            (None, None) => (Utc::now().naive_utc(), true),
        };

        let tariff_class = options.tariff_class.unwrap_or(TariffClass::Second);

        let data = self.request(ijson!({
    		"cfg": {
    			"polyEnc": "GPA"
    		},
    		"meth": "TripSearch",
    		"req": {
    			"ctxScr": null,
    			"getPasslist": options.stopovers.unwrap_or(false),
    			"maxChg": options.transfers.unwrap_or(-1),
    			"minChgTime": options.transfer_time.unwrap_or(0),
    			"numF": options.results.unwrap_or(5),
    			"depLocL": [ from.to_hafas() ],
    			"viaLocL": [],
    			"arrLocL": [ to.to_hafas() ],
    			"jnyFltrL": [
    				{
    					"type": "PROD",
    					"mode": "INC",
    					"value": options.products.unwrap_or_else(|| Products::all()).to_hafas(),
    				},
    				{
    					"type": "META",
    					"mode": "INC",
    					"meta": options.accessibility.unwrap_or(Accessibility::r#None).to_hafas(),
    				}
    			],
    			"gisFltrL": [],
    			"getTariff": options.tickets.unwrap_or(true),
    			"ushrp": options.start_with_walking.unwrap_or(true),
    			"getPT": true,
    			"getIV": false,
    			"getPolyline": options.polylines.unwrap_or(false),
    			"outFrwd": is_departure,
    			"outDate": when.format("%Y%m%d").to_string(),
    			"outTime": when.format("%H%M%S").to_string(),
    			"trfReq": {
    				"jnyCl": tariff_class.to_hafas(),
    				"tvlrProf": [
    					{
    						"type": "E",
    						"redtnCard": null,
    					}
    				],
    				"cType": "PK"
    			}
    		}
        })).await?;

        Ok(parse_journeys_response(data, tariff_class)?)
    }
}
