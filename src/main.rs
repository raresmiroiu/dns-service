mod dns_packet_builder;
mod dns_packet;

use crate::dns_packet_builder::DnsPacketBuilder;
use crate::dns_packet::{DnsPacket, DnsQuestion, DnsResourceRecord};
use crate::dns_packet::error::DnsEncodeError;
fn main() {
    let packet1 = DnsPacketBuilder::new(0x1234)
        .set_flags(0x1000)
        .add_question(DnsQuestion::new("example.com",1,1))
        .build();
    let packet2: DnsPacket = DnsPacketBuilder::new(0xaa12).build();

    // impl pe trait std::fmt::Display ca sa am posibilitatea de a face printare rapida
    let err1: DnsEncodeError = DnsEncodeError::LabelTooLong(67);
    println!("Eroare 1: {err1}");

    let err2: DnsEncodeError = DnsEncodeError::DomainTooLong(7666);
    println!("Eroare 2: {err2}");
}