mod mutate;
mod query;

use std::net::{Ipv6Addr, SocketAddr};

use sellershut_services::Services;

#[derive(Clone)]
pub struct AppState {
    pub addr: SocketAddr,
    pub services: Services,
}

impl AppState {
    pub fn new(port: u16, services: Services) -> Self {
        let listen_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, port));
        Self {
            addr: listen_address,
            services,
        }
    }
}
