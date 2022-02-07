use crate::ParseResult;
use crate::Profile;
use crate::Line;
use crate::Operator;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HafasLineProdCtx {
    num: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HafasLine {
    line: Option<String>,
    add_name: Option<String>,
    name: Option<String>,
    prod_ctx: Option<HafasLineProdCtx>,
    opr_x: Option<usize>,
    cls: Option<u16>,
}

pub(crate) fn default_parse_line<P: Profile>(profile: &P, data: HafasLine, operators: &Vec<Operator>) -> ParseResult<Line> {
    let HafasLine { line, add_name, name, prod_ctx, opr_x, cls } = data;
    let product = profile.parse_product(cls.ok_or_else(|| "Missing cls field")?)?;
    Ok(Line {
        name: line.or(add_name).or(name),
        fahrt_nr: prod_ctx.and_then(|x| x.num.clone()),
        operator: opr_x.and_then(|x| operators.get(x)).cloned(),
        mode: product.mode(),
        product,
    })
}
