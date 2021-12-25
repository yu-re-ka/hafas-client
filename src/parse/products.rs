use crate::Products;

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
