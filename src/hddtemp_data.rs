use super::{HDDTempResult, HDDTempUnits};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct HDDTempData {
    pub model: String,
    pub result: HDDTempResult,
    pub temperature: Option<u64>,
    pub units: Option<HDDTempUnits>,
}
