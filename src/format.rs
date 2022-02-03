use super::{Products, Place, Accessibility, TariffClass, Location, Stop};
use ijson::ijson;

pub trait ToHafas<T> {
    fn to_hafas(&self) -> T;
}

impl ToHafas<u16> for Products {
    fn to_hafas(&self) -> u16 {
        let mut p_cls = 0;
        if self.national_express  { p_cls |= 0b0000_0000_0001; }
        if self.national          { p_cls |= 0b0000_0000_0010; }
        if self.regional_exp      { p_cls |= 0b0000_0000_0100; }
        if self.regional          { p_cls |= 0b0000_0000_1000; }
        if self.suburban          { p_cls |= 0b0000_0001_0000; }
        if self.bus               { p_cls |= 0b0000_0010_0000; }
        if self.ferry             { p_cls |= 0b0000_0100_0000; }
        if self.subway            { p_cls |= 0b0000_1000_0000; }
        if self.tram              { p_cls |= 0b0001_0000_0000; }
        if self.taxi              { p_cls |= 0b0010_0000_0000; }
        p_cls
    }
}

fn format_coord(coordinate: f32) -> u64 {
    (coordinate * 1000000.0) as u64
}

fn format_identifier(components: Vec<(&str, &str)>) -> String {
    components.iter()
        .map(|(k, v)| format!("{}={}@", k, v))
        .collect::<Vec<_>>()
        .join("")
}

impl ToHafas<ijson::IValue> for Place {
    fn to_hafas(&self) -> ijson::IValue {
        match self {
            Place::Stop(stop) => {
                let Stop { id, .. } = stop;
                ijson!({
                    "type": "S",
                    "lid": format_identifier(vec![
                        ("A", "1"),
                        ("L", id),
                    ])
                })
            },
            Place::Location(location) => {
                match location {
                    Location::Address { address, latitude, longitude } => ijson!({
                        "type": "A",
                        "lid": format_identifier(vec![
                            ("A", "2"),
                            ("O", address),
                            ("X", &format_coord(*latitude).to_string()),
                            ("Y", &format_coord(*longitude).to_string()),
                        ])
                    }),
                    Location::Point { id, latitude, longitude, .. } => {
                        let x = format_coord(*latitude).to_string();
                        let y = format_coord(*longitude).to_string();
                        let mut lid = vec![
                            ("A", "4"),
                            ("X", &x),
                            ("Y", &y),
                        ];
                        if let Some(id) = id {
                            lid.push(("L", id));
                        }
                        ijson!({
                            "type": "P",
                            "lid": format_identifier(lid)
                        })
                    },
                }
            },
        }
    }
}

impl ToHafas<String> for Accessibility {
    fn to_hafas(&self) -> String {
        match self {
            Accessibility::r#None => "notBarrierfree",
            Accessibility::Partial => "limitedBarrierfree",
            Accessibility::Complete => "completeBarrierfree",
        }.to_string()
    }
}

impl ToHafas<u64> for TariffClass {
    fn to_hafas(&self) -> u64 {
        match *self {
            TariffClass::First => 1,
            TariffClass::Second => 2,
        }
    }
}
