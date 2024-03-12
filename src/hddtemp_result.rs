use std::fmt;
use std::io::Error;
use std::str::FromStr;

// `ERR` Error
const GETTEMP_ERROR: &str = "ERR";
// `NA`
const GETTEMP_NOT_APPLICABLE: &str = "NA";
// `UNK` Drive is not in database
const GETTEMP_UNKNOWN: &str = "UNK";
// `` Drive appear in database
const GETTEMP_KNOWN: &str = "";
// `NOS` Drive appear in database but is known to have no sensor
const GETTEMP_NOSENSOR: &str = "NOS";
// `SLP` Drive is sleeping
const GETTEMP_DRIVE_SLEEP: &str = "SLP";

/// enum contains error code of hddtemp service query operation
///
/// hddtemp sources for details: `hddtemp/src/daemon.c`,
/// `hddtemp/src/hddtemp.c`, `hddtemp/src/hddtemp.h`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum HDDTempResult {
    /// `ERR` Error
    Error,
    /// `NA`
    NotApplicable,
    /// `UNK` Drive is not in database
    Unknown,
    /// `` Drive appear in database
    #[default]
    Known,
    /// `NOS` Drive appear in database but is known to have no sensor
    NoSensor,
    /// `SLP` Drive is sleeping
    DriveSleep,
}

/// parse HDDTempResult from str
impl FromStr for HDDTempResult {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            GETTEMP_ERROR => Ok(Self::Error),
            GETTEMP_NOT_APPLICABLE => Ok(Self::NotApplicable),
            GETTEMP_UNKNOWN => Ok(Self::Unknown),
            GETTEMP_NOSENSOR => Ok(Self::NoSensor),
            GETTEMP_DRIVE_SLEEP => Ok(Self::DriveSleep),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Wrong error code.",
            )),
        }
    }
}

/// convert HDDTempResult to str
impl fmt::Display for HDDTempResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Error => GETTEMP_ERROR,
            Self::NotApplicable => GETTEMP_NOT_APPLICABLE,
            Self::Unknown => GETTEMP_UNKNOWN,
            Self::Known => GETTEMP_KNOWN,
            Self::NoSensor => GETTEMP_NOSENSOR,
            Self::DriveSleep => GETTEMP_DRIVE_SLEEP,
        };
        write!(f, "{}", s)
    }
}
