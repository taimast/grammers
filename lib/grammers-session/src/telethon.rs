use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::generated::types;
use crate::generated::types::DataCenter;
use crate::Session;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TelethonSession {
    pub dc_id: i32,
    pub ip: String,
    pub port: i32,
    pub auth_key: Vec<u8>,
}


impl Into<Session> for TelethonSession {
    fn into(self) -> Session {
        let ipv4 = self.ip.parse::<Ipv4Addr>().ok();
        let ipv6 = self.ip.parse::<Ipv6Addr>().ok();

        let dc = DataCenter {
            id: self.dc_id,
            ipv4: ipv4.map(|addr| i32::from_le_bytes(addr.octets())),
            ipv6: ipv6.map(|addr| addr.octets()),
            port: self.port,
            auth: Some(self.auth_key.clone()),
        };
        Session {
            session: Mutex::new(types::Session {
                dcs: vec![dc.into()],
                user: None,
                state: None,
            }),
        }
    }
}
