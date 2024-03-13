use super::{HDDTempData, HDDTempResult};
use std::collections::BTreeMap;
use std::io::{Error, ErrorKind};

// error marker of
pub const ASTERISK: &str = "*";

// INFO: hddtemp/README
// `?` or `*` not allowed, can be in output response of hddtemp
pub const SEPARATOR_DEFAULT: char = '|';

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct HDDTempParser;

impl HDDTempParser {
    /// parse devices and save device [`HDDTempData`] in [`BTreeMap`]
    ///
    /// Example input string:
    ///
    /// `|dev_1|model_1|temp_1|unit_1||dev_2|model_2|temp_2|unit2||...||dev_n|model_n|temp_n|unit_n|`
    pub fn parse_devices<S: AsRef<str>>(
        s: S,
        separator: Option<char>,
    ) -> Result<BTreeMap<String, HDDTempData>, Error> {
        let s = s.as_ref();
        let separator = separator.unwrap_or(SEPARATOR_DEFAULT);
        let double_separator = format!("{}{}", separator, separator);

        // if first and last chars are separators
        if let Some(s) = s.strip_prefix(separator) {
            if let Some(s) = s.strip_suffix(separator) {
                let lines: Vec<&str> = s.split(&double_separator).collect();
                let mut devices: BTreeMap<String, HDDTempData> = BTreeMap::new();

                for line in lines {
                    let (device, data) = HDDTempParser::parse_device(line, Some(separator))?;
                    devices.insert(device, data);
                }

                Ok(devices)
            } else {
                Err(Error::new(
                    ErrorKind::InvalidData,
                    "Trailing separator not found.",
                ))
            }
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "Starting separator not found.",
            ))
        }
    }

    /// parse single device string (without starting and ending `|`)
    ///
    /// Example input strings:
    ///
    ///  * `dev_1|model_1|temp_1|unit_1` - success, contains temperature and units
    ///  * `dev_2|model_2|ERR|*` - error, contains error type
    ///
    pub fn parse_device<S: AsRef<str>>(
        s: S,
        separator: Option<char>,
    ) -> Result<(String, HDDTempData), Error> {
        let s = s.as_ref();
        let separator = separator.unwrap_or(SEPARATOR_DEFAULT);

        let columns: Vec<&str> = s.split(separator).collect();

        let device = columns.first().map(|s| s.to_string()).unwrap_or_default();
        let mut data = HDDTempData {
            model: columns.get(1).map(|s| s.to_string()).unwrap_or_default(),
            ..Default::default()
        };

        // `*` - in column as marker for an error, in previous column is stored the error type
        if columns.get(3) == Some(&ASTERISK) {
            data.result = columns
                .get(2)
                .and_then(|s| s.parse::<HDDTempResult>().ok())
                .unwrap_or_default();
        } else {
            data.temperature = columns.get(2).and_then(|s| s.parse().ok());
            data.units = columns.get(3).and_then(|s| s.parse().ok());
            data.result = HDDTempResult::Known;
        }

        Ok((device, data))
    }
}

#[test]
fn parse_devices_test() {
    use crate::HDDTempUnits;

    let s = "|/dev/sda|ABCDEFGHIJ|20|C||/dev/sdb|abcdefghij ABCDEFGHIJ|20|C|";
    let orig = BTreeMap::from([
        (
            "/dev/sda".to_string(),
            HDDTempData {
                model: "ABCDEFGHIJ".to_string(),
                result: HDDTempResult::Known,
                temperature: Some(20),
                units: Some(HDDTempUnits::Celsius),
            },
        ),
        (
            "/dev/sdb".to_string(),
            HDDTempData {
                model: "abcdefghij ABCDEFGHIJ".to_string(),
                result: HDDTempResult::Known,
                temperature: Some(20),
                units: Some(HDDTempUnits::Celsius),
            },
        ),
    ]);

    let devices = HDDTempParser::parse_devices(&s, None).unwrap();
    assert_eq!(orig, devices);
}

#[test]
fn parse_device_test() {
    use crate::HDDTempUnits;

    let s = "/dev/sda|ABCDEFGHIJ|20|C";
    let orig = (
        "/dev/sda".to_string(),
        HDDTempData {
            model: "ABCDEFGHIJ".to_string(),
            result: HDDTempResult::Known,
            temperature: Some(20),
            units: Some(HDDTempUnits::Celsius),
        },
    );
    let r = HDDTempParser::parse_device(&s, None).unwrap();
    assert_eq!(orig, r);

    let s = "/dev/sdb|abcdefghij ABCDEFGHIJ|20|C";
    let orig = (
        "/dev/sdb".to_string(),
        HDDTempData {
            model: "abcdefghij ABCDEFGHIJ".to_string(),
            result: HDDTempResult::Known,
            temperature: Some(20),
            units: Some(HDDTempUnits::Celsius),
        },
    );
    let r = HDDTempParser::parse_device(&s, None).unwrap();
    assert_eq!(orig, r);

    let s = "device|model|NA|*";
    let orig = (
        "device".to_string(),
        HDDTempData {
            model: "model".to_string(),
            result: HDDTempResult::NotApplicable,
            temperature: None,
            units: None,
        },
    );
    let r = HDDTempParser::parse_device(&s, None).unwrap();
    assert_eq!(orig, r);

    let s = "device|model|UNK|*";
    let orig = (
        "device".to_string(),
        HDDTempData {
            model: "model".to_string(),
            result: HDDTempResult::Unknown,
            temperature: None,
            units: None,
        },
    );
    let r = HDDTempParser::parse_device(&s, None).unwrap();
    assert_eq!(orig, r);

    let s = "device|model|20|C";
    let orig = (
        "device".to_string(),
        HDDTempData {
            model: "model".to_string(),
            result: HDDTempResult::Known,
            temperature: Some(20),
            units: Some(HDDTempUnits::Celsius),
        },
    );
    let r = HDDTempParser::parse_device(&s, None).unwrap();
    assert_eq!(orig, r);

    let s = "device|model|NOS|*";
    let orig = (
        "device".to_string(),
        HDDTempData {
            model: "model".to_string(),
            result: HDDTempResult::NoSensor,
            temperature: None,
            units: None,
        },
    );
    let r = HDDTempParser::parse_device(&s, None).unwrap();
    assert_eq!(orig, r);

    let s = "device|model|SLP|*";
    let orig = (
        "device".to_string(),
        HDDTempData {
            model: "model".to_string(),
            result: HDDTempResult::DriveSleep,
            temperature: None,
            units: None,
        },
    );
    let r = HDDTempParser::parse_device(&s, None).unwrap();
    assert_eq!(orig, r);

    let s = "device|???|ERR|*";
    let orig = (
        "device".to_string(),
        HDDTempData {
            model: "???".to_string(),
            result: HDDTempResult::Error,
            temperature: None,
            units: None,
        },
    );
    let r = HDDTempParser::parse_device(&s, None).unwrap();
    assert_eq!(orig, r);
}
