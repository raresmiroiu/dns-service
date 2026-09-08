mod dns_packet_builder;
mod dns_packet;

use crate::dns_packet_builder::DnsPacketBuilder;
use crate::dns_packet::{DnsPacket, DnsQuestion, DnsEncodeError};
fn main() {
    let packet1 = DnsPacketBuilder::new(0x1234)
        .set_flags(0x1000)
        .add_question(DnsQuestion::new("example.com",1,1))
        .build();
    let packet2: DnsPacket = DnsPacketBuilder::new(0xaa12).build();

    let err = DnsEncodeError::LabelTooLong(67);
    println!("Eroare: {err}");
}
