use crate::ParseResult;
use crate::Operator;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HafasOperator {
    name: String,
}

pub(crate) fn default_parse_operator(data: HafasOperator) -> ParseResult<Operator> {
    let HafasOperator { name } = data;
    Ok(Operator {
        id: name.clone(), // FIXME
        name,
    })
}
