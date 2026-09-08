mod DnsHeader;
mod DnsQuestion;
mod DnsResourceRecord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DnsHeader{
    header: DnsHeader,
    question: DnsQuestion,
    answers: Vec<DnsResourceRecord>,
}