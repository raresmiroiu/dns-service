#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion{
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16
}

impl DnsQuestion{
    pub fn new(qname: &str, qtype: u16, qclass: u16) ->Self{
        Self{
            qname: qname.to_string(),
            qtype: qtype,
            qclass: qclass,
        }
    }
}