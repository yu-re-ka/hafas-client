use crate::TariffClass;
use crate::Result;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use crate::LoadFactor;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HafasTariffClass {
    First,
    Second,
}

impl From<HafasTariffClass> for TariffClass {
    fn from(h: HafasTariffClass) -> TariffClass {
        match h {
            HafasTariffClass::First => TariffClass::First,
            HafasTariffClass::Second => TariffClass::Second,
        }
    }
}

#[derive(Debug, Clone, Deserialize_repr)]
#[repr(u8)]
pub enum HafasLoadFactor {
    LowToMedium = 1,
    High = 2,
    VeryHigh = 3,
    ExceptionallyHigh = 4,
}

impl From<HafasLoadFactor> for LoadFactor {
    fn from(h: HafasLoadFactor) -> LoadFactor {
        match h {
            HafasLoadFactor::LowToMedium => LoadFactor::LowToMedium,
            HafasLoadFactor::High => LoadFactor::High,
            HafasLoadFactor::VeryHigh => LoadFactor::VeryHigh,
            HafasLoadFactor::ExceptionallyHigh => LoadFactor::ExceptionallyHigh,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct HafasLoadFactorEntry {
    c: HafasTariffClass,
    r: HafasLoadFactor,
}

#[derive(Debug, Clone)]
pub struct LoadFactorEntry {
    pub class: TariffClass,
    pub load: LoadFactor,
}

impl From<HafasLoadFactorEntry> for LoadFactorEntry {
    fn from(h: HafasLoadFactorEntry) -> LoadFactorEntry {
        LoadFactorEntry {
            class: h.c.into(),
            load: h.r.into(),
        }
    }
}
