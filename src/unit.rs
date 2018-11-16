use std::string;
use std::fmt;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Unit {
    Byte,
    Kilo,
    Mega,
    Giga,
    Tera,
}

pub enum UnitValue {
    Byte = 1,
    Kilo = 1 << 10,
    Mega = 1 << 20,
    Giga = 1 << 30,
    Tera = 1 << 40,
}

impl Unit {
    fn to_value(&self) -> u64 {
        let value = match self {
            Unit::Byte => UnitValue::Byte,
            Unit::Kilo => UnitValue::Kilo,
            Unit::Mega => UnitValue::Mega,
            Unit::Giga => UnitValue::Giga,
            Unit::Tera => UnitValue::Tera,
        };

        value as u64
    }
}

impl string::ToString for Unit {
    fn to_string(&self) -> String {
        match self {
            Unit::Byte => "",
            Unit::Kilo => "Ki",
            Unit::Mega => "Mi",
            Unit::Giga => "Gi",
            Unit::Tera => "Ti"
        }.to_string()
    }
}

pub struct Amount {
    bytes: f64,
    unit: Unit
}

impl Amount {
    pub fn new(bytes: f64, unit: Unit) -> Amount {
        Amount { bytes, unit }
    }

    pub fn auto_detect(bytes: f64) -> Amount {
        let scales: [Unit; 5] = [Unit::Byte, Unit::Kilo, Unit::Mega, Unit::Giga, Unit::Tera];
        let mut amount = bytes;
        let mut counter = 0;

        while amount > 1.0 && counter < 5 {
            amount = amount / 1024.0;
            counter += 1
        }

        Self::new(bytes, scales[counter - 1])
    }
}

impl Amount {
    fn to_quantity(&self) -> f64 {
        self.bytes / (self.unit.to_value() as f64)
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1} {}B", self.to_quantity(), self.unit.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::{Unit, Amount};

    #[test]
    fn unit_to_string() {
        let unit = Unit::Giga;

        assert_eq!(unit.to_string(), String::from("Gi"));
    }

    #[test]
    fn amount_new() {
        let amount = Amount::new(100.0, Unit::Giga);

        assert_eq!(amount.bytes, 100.0);
        assert_eq!(amount.unit, Unit::Giga);
    }

    #[test]
    fn amount_auto_detect() {
        assert_eq!(Amount::auto_detect(42.0).unit, Unit::Byte);
        assert_eq!(Amount::auto_detect(2048.0).unit, Unit::Kilo);
        assert_eq!(Amount::auto_detect(1234567.0).unit, Unit::Mega);
        assert_eq!(Amount::auto_detect(1234567890.0).unit, Unit::Giga);
        assert_eq!(Amount::auto_detect(1234567890123.0).unit, Unit::Tera);
    }

    #[test]
    fn amount_display() {
        assert_eq!(format!("{}", Amount::auto_detect(42.0)), "42.0 B");
        assert_eq!(format!("{}", Amount::auto_detect(200124.42)), "195.4 KiB");
    }
}
