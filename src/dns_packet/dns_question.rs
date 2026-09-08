#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion{
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16
}