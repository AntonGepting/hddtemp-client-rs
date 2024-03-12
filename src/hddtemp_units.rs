use std::fmt;
use std::io::{Error, ErrorKind};
use std::str::FromStr;

/// units for temperature
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum HDDTempUnits {
    #[default]
    /// `C`
    Celsius,
    /// `F`
    Fahrenheit,
}

const CELSIUS: char = 'C';
const FAHRENHEIT: char = 'F';

impl FromStr for HDDTempUnits {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0) {
            Some(CELSIUS) => Ok(Self::Celsius),
            Some(FAHRENHEIT) => Ok(Self::Fahrenheit),
            _ => Err(Error::new(ErrorKind::InvalidData, "")),
        }
    }
}

impl fmt::Display for HDDTempUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Celsius => CELSIUS,
            Self::Fahrenheit => FAHRENHEIT,
        };
        write!(f, "{}", s)
    }
}
