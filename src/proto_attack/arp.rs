use std::net::Ipv4Addr;

use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::{MutablePacket, Packet};
use pnet::packet::arp::{ArpHardwareTypes, ArpOperation, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use pnet::util::MacAddr;

pub mod spoof;

#[derive(Debug, Eq, PartialEq)]
pub struct Arp {
    source_mac: MacAddr,
    source_ip: Ipv4Addr,
    target_mac: MacAddr,
    target_ip: Ipv4Addr,
    operation: ArpOperation,

}

trait ArpBuilder {
    fn new(source_mac: MacAddr, source_ip: Ipv4Addr, target_mac: MacAddr, target_ip: Ipv4Addr, operation: ArpOperation) -> Arp {
        Arp {
            source_mac,
            source_ip,
            target_mac,
            target_ip,
            operation,
        }
    }

    fn broadcast(source_mac: MacAddr, source_ip: Ipv4Addr, target_ip: Ipv4Addr, operation: ArpOperation) -> Arp {
        Self::new(source_mac, source_ip, MacAddr::broadcast(), target_ip, operation)
    }
}

pub trait Sender {
    fn send(&self, inter_name: &str);
}

impl ArpBuilder for Arp {}

impl Sender for Arp {
    fn send(&self, inter_name: &str) {
        let mut ethernet_buffer = [0u8; 42]; // ARP包大小
        let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

        let mut arp_buffer = [0u8; 28];
        let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

        // 构造Ethernet头
        ethernet_packet.set_destination(self.target_mac);
        ethernet_packet.set_source(self.source_mac);
        ethernet_packet.set_ethertype(EtherTypes::Arp);

        // 构造ARP包
        arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
        arp_packet.set_protocol_type(EtherTypes::Ipv4);
        arp_packet.set_hw_addr_len(6);
        arp_packet.set_proto_addr_len(4);
        arp_packet.set_operation(self.operation);
        arp_packet.set_sender_hw_addr(self.source_mac);
        arp_packet.set_sender_proto_addr(self.source_ip);
        arp_packet.set_target_hw_addr(self.target_mac);
        arp_packet.set_target_proto_addr(self.target_ip);

        ethernet_packet.set_payload(arp_packet.packet_mut());

        let interfaces = datalink::interfaces();
        let interface = interfaces
            .into_iter()
            .filter(|inter| !inter.is_loopback() && inter.is_running() && inter.name.eq(inter_name))
            .next()
            .expect("Failed to get network interface");
        let (mut tx, _) = match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("不支持的通道类型"),
            Err(e) => panic!("创建通道时发生错误: {:?}", e),
        };
        tx.send_to(ethernet_packet.packet(), None).unwrap().expect("无法发送ARP请求");
    }
}