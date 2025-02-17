mod mutate;
mod query;

use std::net::{Ipv6Addr, SocketAddr};

use sellershut_services::Services;

use crate::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub addr: SocketAddr,
    pub services: Services,
    pub config: AppConfig,
}

impl AppState {
    pub fn new(port: u16, services: Services, config: AppConfig) -> Self {
        let listen_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, port));
        Self {
            addr: listen_address,
            services,
            config,
        }
    }
}
