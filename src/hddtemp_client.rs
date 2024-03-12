use super::{HDDTempData, HDDTempParser};
use std::collections::BTreeMap;
use std::io::{Error, Read};
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

// XXX: newtype or default types use?
//#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
//pub struct HDDTemp(pub BTreeMap<String, HDDTempData>);

// XXX: structure for self or functions arguments?
/// Client structure for connecting and requesting temperature from server
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct HDDTempClient;
// #[derive(Debug, Clone, Default)]
// pub struct HDDTempClient {
//     pub timeout: Duration,
//     // `src/hddtemp.h`
//     pub separator: char,
// }

impl HDDTempClient {
    /// connect and read devices temperatures string from server for later parse and use
    ///
    /// address - [`ToSocketAddrs`] server address and port
    /// (default port: 7634)
    ///
    /// timeout - [`Duration`] in milliseconds
    ///
    pub fn connect_read_to_string<A: ToSocketAddrs>(
        address: A,
        timeout: Option<Duration>,
    ) -> Result<String, Error> {
        let mut stream = TcpStream::connect(address)?;
        stream.set_read_timeout(timeout)?;

        let mut s = String::new();
        stream.read_to_string(&mut s)?;

        Ok(s)
    }

    /// get temperature data from server
    ///
    /// address - [`ToSocketAddrs`] server address and port
    ///
    /// timeout - [`Duration`] in milliseconds
    ///
    /// separator - [`char`] optional custom symbol (default `|`, hddtemp sources for details `src/hddtemp.h`)
    ///
    /// result - [`BTreeMap`] containing device name with corresponding [`HDDTempData`]
    pub fn get<A: ToSocketAddrs>(
        address: A,
        timeout: Option<Duration>,
        separator: Option<char>,
    ) -> Result<BTreeMap<String, HDDTempData>, Error> {
        let s = Self::connect_read_to_string(address, timeout)?;

        HDDTempParser::parse_devices(s, separator)
    }
}

#[test]
fn get_hdd_temp_test() {
    let s = HDDTempClient::connect_read_to_string("localhost:7634", None).unwrap();
    dbg!(s);
    let devices = HDDTempClient::get("localhost:7634", None, None).unwrap();
    dbg!(devices);
}
