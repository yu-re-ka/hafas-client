use crate::Result;
use crate::Error;
use crate::parse::journeys_response::CommonData;
use crate::Journey;
use chrono::NaiveDate;
use serde::Deserialize;
use crate::parse::leg::{HafasLeg, parse_leg};

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct HafasJourney {
    date: String,
    ctx_recon: Option<String>,
    sec_l: Vec<HafasLeg>,
}

pub fn parse_journey(data: HafasJourney, common: &CommonData) -> Result<Journey> {
    let HafasJourney { date, ctx_recon, sec_l } = data;

    let date = NaiveDate::parse_from_str(&date, "%Y%m%d").map_err(|_| Error::InvalidData)?;

    /*if j{"trfRes"}{"statusCode"}.getStr == "OK":
      result.price = some(Price(
        amount: j["trfRes"]["fareSetL"][0]["fareL"][0]["prc"].getInt / 100,
        currency: some("Euro"),
      ))*/

    Ok(Journey {
        refresh_token: ctx_recon,
        legs: sec_l.into_iter().map(|x| parse_leg(x, common, &date)).collect::<Result<_>>()?,
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
