use crate::ParseResult;
use crate::Line;
use crate::Operator;
use serde::Deserialize;
use crate::parse::products::parse_product;

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
    cls: u16,
}

pub fn parse_line(data: HafasLine, operators: &Vec<Operator>) -> ParseResult<Line> {
    let HafasLine { line, add_name, name, prod_ctx, opr_x, cls } = data;
    let product = parse_product(cls)?;
    Ok(Line {
        name: line.or(add_name).or(name),
        fahrt_nr: prod_ctx.and_then(|x| x.num.clone()),
        operator: opr_x.and_then(|x| operators.get(x)).cloned(),
        mode: product.mode(),
        product,
    })
}
