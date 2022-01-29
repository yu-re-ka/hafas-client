use crate::Result;
use crate::Error;
use crate::Remark;
use crate::RemarkType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasRemark {
    r#type: Option<String>,
    txt_s: Option<String>,
    txt_n: String,
    code: Option<String>,
    jid: Option<String>,
}

pub fn parse_remark(rem: HafasRemark) -> Result<Remark> {
    Ok(match &rem.r#type.unwrap_or("other".to_string()).as_str() {
        &"M" | &"P" => Remark {
            r#type: RemarkType::Status,
            code: rem.code.ok_or(Error::InvalidData)?,
            text: rem.txt_n,
            trip_id: None,
            summary: rem.txt_s,
        },
        &"L" => Remark {
            r#type: RemarkType::Status,
            code: "alternative-trip".to_string(),
            text: rem.txt_n,
            trip_id: rem.jid,
            summary: None,
        },
        &"A" | &"I" | &"H" => Remark {
            r#type: RemarkType::Hint,
            code: rem.code.ok_or(Error::InvalidData)?,
            text: rem.txt_n,
            trip_id: None,
            summary: None,
        },
        _ => Remark {
            // TODO: parse more accurately
            r#type: RemarkType::Status,
            code: rem.code.ok_or(Error::InvalidData)?,
            text: rem.txt_n,
            trip_id: None,
            summary: None,
        },
    })
}
