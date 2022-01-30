use crate::ParseResult;
use crate::Products;
use crate::Product;

pub fn parse_products(p_cls: u16) -> Products {
    Products {
        national_exp: p_cls & 0b0000_0000_0001 != 0,
        national:     p_cls & 0b0000_0000_0010 != 0,
        regional_exp: p_cls & 0b0000_0000_0100 != 0,
        regional:     p_cls & 0b0000_0000_1000 != 0,
        suburban:     p_cls & 0b0000_0001_0000 != 0,
        bus:          p_cls & 0b0000_0010_0000 != 0,
        ferry:        p_cls & 0b0000_0100_0000 != 0,
        subway:       p_cls & 0b0000_1000_0000 != 0,
        tram:         p_cls & 0b0001_0000_0000 != 0,
        taxi:         p_cls & 0b0010_0000_0000 != 0,
    }
}

pub fn parse_product(p_cls: u16) -> ParseResult<Product> {
    Ok(match p_cls {
         0b0000_0000_0001 => Product::NationalExp,
         0b0000_0000_0010 => Product::National,
         0b0000_0000_0100 => Product::RegionalExp,
         0b0000_0000_1000 => Product::Regional,
         0b0000_0001_0000 => Product::Suburban,
         0b0000_0010_0000 => Product::Bus,
         0b0000_0100_0000 => Product::Ferry,
         0b0000_1000_0000 => Product::Subway,
         0b0001_0000_0000 => Product::Tram,
         0b0010_0000_0000 => Product::Taxi,
         _ => return Err(format!("Unknown product bit: {:b}", p_cls).into()),
    })
}
