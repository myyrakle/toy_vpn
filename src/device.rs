use std::{
    io,
    net::{SocketAddr, UdpSocket},
};

use tun_tap::Iface;

use crate::peer::Peer;

pub struct Device {
    udp: UdpSocket,
    iface: Iface,
    pub(crate) peer: Peer,
}

impl Device {
    fn loop_listen_iface(&self) -> io::Result<()> {
        // a large enough buffer, recall the MTU on iface was to be set to 1472

        let mut buf = [0u8; 1504];

        loop {
            let nbytes = self.iface.recv(&mut buf[..])?;

            let peer = self.peer.endpoint();

            if let Some(peer_addr) = peer.as_ref() {
                self.udp.send_to(&buf[..nbytes], peer_addr)?;
            } else {
                eprintln!("..no peer");
            }
        }
    }
}

impl Device {
    fn loop_listen_udp(&self) -> io::Result<()> {
        let mut buf = [0u8; 1504];

        loop {
            let (nbytes, peer_addr) = self.udp.recv_from(&mut buf[..])?;

            if let SocketAddr::V4(peer_addr_v4) = peer_addr {
                self.peer.set_endpoint(peer_addr_v4);

                self.iface.send(&buf[..nbytes])?;
            }
        }
    }
}
