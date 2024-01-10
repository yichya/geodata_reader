use ipnet::{IpNet, PrefixLenError};
use prost::Message;
use std::net::{Ipv4Addr, Ipv6Addr};

include!(concat!(env!("OUT_DIR"), "/geodata.rs"));

pub fn deserialize_geoip(buf: &[u8]) -> Result<GeoIpList, prost::DecodeError> {
    GeoIpList::decode(buf)
}

pub fn deserialize_geosite(buf: &[u8]) -> Result<GeoSiteList, prost::DecodeError> {
    GeoSiteList::decode(buf)
}

impl Cidr {
    pub fn to_ipnet(&self) -> Result<IpNet, PrefixLenError> {
        let prefix_len = self.prefix as u8;
        match self.ip.len() {
            4 => match self.ip[0..4].try_into() as Result<[u8; 4], _> {
                Ok(seg4) => IpNet::new(Ipv4Addr::from(seg4).into(), prefix_len),
                Err(_) => Err(PrefixLenError),
            },
            16 => match self.ip[0..16].try_into() as Result<[u8; 16], _> {
                Ok(seg16) => IpNet::new(Ipv6Addr::from(seg16).into(), prefix_len),
                Err(_) => Err(PrefixLenError),
            },
            _ => Err(PrefixLenError),
        }
    }
}
