pub struct JourneysResponse {
    pub earlierRef: Option<String>,
    pub laterRef: Option<String>,
    pub journeys: Vec<Journey>,

pub fn parse_journeys_response() -> Result<JourneysResponse> {
    //let points = map(data["res"]["common"]["locL"].getElems(), parsePoint)
    //let operators = map(data["res"]["common"]["opL"].getElems(), parseOperator)
    //let remarks = map(data["res"]["common"]["remL"].getElems(), parseRemark)
    //let lines = data["res"]["common"]["prodL"]
    //let polylines = map(data["res"]["common"]["polyL"].getElems(), mkParsePolyline(points))
    //let timestamp = parseInt(data["res"]["planrtTS"].getStr())
    //let common = CommonData(points: points, operators: operators, remarks: remarks, lines: lines, polylines: polylines, timestamp: timestamp)

    //result.journeys = data["res"]["outConL"].getElems().map(mkParseJourney(common))

    Ok(JourneysResponse {
        earlierRef: data["res"]["outCtxScrB"].get_str().to_owned(),
        laterRef: data["res"]["outCtxScrF"].get_str().to_owned(),
    })
}
