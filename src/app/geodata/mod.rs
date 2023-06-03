use prost::Message;

include!(concat!(env!("OUT_DIR"), "/geodata.rs"));

#[allow(dead_code)]
pub fn deserialize_geoip(buf: &[u8]) -> Result<GeoIpList, prost::DecodeError> {
    GeoIpList::decode(buf)
}

#[allow(dead_code)]
pub fn deserialize_geosite(buf: &[u8]) -> Result<GeoSiteList, prost::DecodeError> {
    GeoSiteList::decode(buf)
}
