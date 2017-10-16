extern crate hwaddr;

use std::net::IpAddr;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::error::Error;
use hwaddr::HwAddr;

pub struct ArpEntry {
    pub ip: IpAddr,
    pub mac: HwAddr,
}

const ARP_FILENAME: &'static str = "/proc/net/arp";

pub fn get_arp_table() -> io::Result<Vec<ArpEntry>> {
    let arp_file = File::open(ARP_FILENAME)?;
    let arp_reader = BufReader::new(arp_file);

    let mut ret = vec![];

    for line in arp_reader.lines().skip(1) {
        let line = line?;

        let mut line_components = line.split_whitespace();

        let ip_str = line_components.nth(0).ok_or(io::Error::new(io::ErrorKind::InvalidData, "Could not find ip"))?;
        let mac_str = line_components.nth(3).ok_or(io::Error::new(io::ErrorKind::InvalidData, "Coudl not find mac"))?;

        let ip =
            ip_str.parse::<IpAddr>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.description().to_string()))?;
        let mac =
            mac_str.parse::<HwAddr>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.description().to_string()))?;

        ret.push(ArpEntry { ip, mac })
    }

    Ok(ret)
}
