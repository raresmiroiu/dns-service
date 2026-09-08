#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsResourceRecord{
    pub rr_name: String,
    pub rr_type: u16,
    pub rr_class: u16,
    pub rr_ttl: u32,
    pub rdata: Vec<u8>,
}

impl DnsResourceRecord {
    pub fn new(rr_name: &str, rr_type: u16, rr_class: u16, rr_ttl: u32, rdata: Vec<u8>)-> Self{
        Self {
            rr_name: rr_name.to_string(), 
            rr_type: rr_type, 
            rr_class: rr_class, 
            rr_ttl: rr_ttl, 
            rdata: rdata, 
        }
    }
    pub fn get_rdlength(&self) -> u16{
        self.rdata.len() as u16
    }

}