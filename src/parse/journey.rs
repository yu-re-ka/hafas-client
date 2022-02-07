use crate::ParseResult;
use crate::Profile;
use crate::Price;
use crate::parse::common::CommonData;
use crate::Journey;
use chrono::NaiveDate;
use serde::Deserialize;
use crate::parse::leg::HafasLeg;

#[derive(Debug, Deserialize)]
pub struct HafasJourneyFare {
    prc: Option<i64>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct HafasJourneyFareSet {
    fare_l: Vec<HafasJourneyFare>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct HafasJourneyTrfRes {
    fare_set_l: Vec<HafasJourneyFareSet>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct HafasJourney {
    date: String,
    ctx_recon: Option<String>,
    sec_l: Vec<HafasLeg>,
    trf_res: Option<HafasJourneyTrfRes>,
}

pub(crate) fn default_parse_journey<P: Profile>(profile: &P, data: HafasJourney, common: &CommonData) -> ParseResult<Journey> {
    let HafasJourney { date, ctx_recon, sec_l, trf_res } = data;

    let date = NaiveDate::parse_from_str(&date, "%Y%m%d")?;

    let lowest_price = trf_res.map(|x| x.fare_set_l).unwrap_or(vec![])
        .into_iter()
        .flat_map(|x| x.fare_l)
        .filter_map(|x| x.prc)
        .filter(|x| *x > 0)
        .min()
        .map(|x| Price {
            currency: profile.price_currency().to_string(),
            amount: x as f64 / 100.0,
        });

    Ok(Journey {
        refresh_token: ctx_recon,
        legs: sec_l.into_iter().map(|x| profile.parse_leg(x, common, &date)).collect::<ParseResult<_>>()?,
        price: lowest_price,
    })

    /*# combine walking legs
    var i = -1
    var firstWalking = -1
    while true:
      inc(i)
      if i >= len(result.legs): break
      if result.legs[i].isWalking:
        if firstWalking == -1:
          firstWalking = i
        else:
          result.legs[firstWalking].arrival = result.legs[i].arrival
          result.legs[firstWalking].distance.get += result.legs[i].distance.get
          result.legs.delete(i)
          dec(i)
      else:
        firstWalking = -1*/
}
