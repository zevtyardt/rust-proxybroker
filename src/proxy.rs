use std::net::IpAddr;

use anyhow::Context;

use crate::{
    error_context,
    geolite::{lookup::geo_lookup, GeoData},
};

#[derive(Debug)]
pub struct Proxy {
    host: String,
    port: u16,
    geo: GeoData,
    types: Vec<String>,
    is_working: bool,
}

impl Proxy {
    pub async fn new(host: impl Into<Host>, port: u16) -> anyhow::Result<Self> {
        let host_str = match host.into() {
            Host::Ip(ip) => ip.to_string(),
            Host::Str(ip) => ip,
        };

        let ip = host_str.parse::<IpAddr>().context(error_context!())?;
        let geo = geo_lookup(ip).await.context(error_context!())?;
        Ok(Self {
            host: host_str,
            port,
            geo,
            types: vec![],
            is_working: false,
        })
    }
}

impl std::fmt::Display for Proxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "<Proxy {} [{}] {}:{}>",
            self.geo.iso_code.to_uppercase(),
            self.types.join(", "),
            self.host,
            self.port
        )
    }
}

pub enum Host {
    Ip(IpAddr),
    Str(String),
}

impl From<IpAddr> for Host {
    fn from(ip: IpAddr) -> Self {
        Host::Ip(ip)
    }
}

impl<'a> From<&'a str> for Host {
    fn from(s: &'a str) -> Self {
        Host::Str(s.to_owned())
    }
}
