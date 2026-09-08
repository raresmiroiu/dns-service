pub mod dns_header;
pub mod dns_question;
pub mod dns_resource_record;

use crate::dns_header::DnsHeader;
use crate::dns_question::DnsQuestion;
use crate::dns_resource_record::DnsResourceRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsPacket{
    header: DnsHeader,
    questions: Vec<DnsQuestion>,
    answers: Vec<DnsResourceRecord>,
}