use std::{
    net::{SocketAddrV4, UdpSocket},
    sync::Mutex,
};

pub struct Peer {
    endpoint: Mutex<Option<SocketAddrV4>>,
}

impl Peer {
    pub fn endpoint(&self) -> MutexGuard<Option<SocketAddrV4>> {
        self.endpoint.lock()
    }

    pub fn set_endpoint(&self, addr: SocketAddrV4) {
        let mut endpoint = self.endpoint.lock();

        if endpoint.is_none() {
            *endpoint = Some(addr);
        }
    }
}
