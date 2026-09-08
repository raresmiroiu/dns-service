pub mod dns_header;
pub mod dns_question;
pub mod dns_resource_record;

use dns_header::DnsHeader;
use dns_question::DnsQuestion;
use dns_resource_record::DnsResourceRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsPacket{
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsResourceRecord>,
    pub authorities: Vec<DnsResourceRecord>,
    pub additionals: Vec<DnsResourceRecord>,
}