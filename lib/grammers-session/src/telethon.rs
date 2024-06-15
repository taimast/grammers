use std::io;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::sync::Mutex;

use base64::Engine;
use base64::prelude::BASE64_URL_SAFE;
use serde::{Deserialize, Serialize};

use crate::generated::types;
use crate::generated::types::DataCenter;
use crate::Session;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TelethonSession {
    pub dc_id: i32,
    pub ip: String,
    pub port: i32,
    pub auth_key: String,
}


impl TryInto<Session> for TelethonSession {
    type Error = io::Error;
    fn try_into(self) -> Result<Session, Self::Error> {
        let ipv4 = self.ip.parse::<Ipv4Addr>().ok();
        let ipv6 = self.ip.parse::<Ipv6Addr>().ok();

        let auth = BASE64_URL_SAFE.decode(self.auth_key).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, e)
        })?;

        let dc = DataCenter {
            id: self.dc_id,
            ipv4: ipv4.map(|addr| i32::from_le_bytes(addr.octets())),
            ipv6: ipv6.map(|addr| addr.octets()),
            port: self.port,
            auth: Some(auth),
        };
        Ok(Session {
            session: Mutex::new(types::Session {
                dcs: vec![dc.into()],
                user: None,
                state: None,
            }),
        })
    }
}
