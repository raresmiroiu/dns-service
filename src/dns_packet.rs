pub mod dns_header;
pub mod dns_question;
pub mod dns_resource_record;
pub mod error;

pub use dns_header::DnsHeader;
pub use dns_question::DnsQuestion;
pub use dns_resource_record::DnsResourceRecord;
pub use error::DnsEncodeError;

//bazat rfc 1035 
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsPacket{
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsResourceRecord>,
    pub authorities: Vec<DnsResourceRecord>,
    pub additionals: Vec<DnsResourceRecord>,
}