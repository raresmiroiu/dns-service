#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DnsResourceRecord{
    pub rr_name: String,
    pub rr_type: u16,
    pub rr_class: u16,
    pub rr_ttl: u32,
    pub rdata: Vec<u8>,
}