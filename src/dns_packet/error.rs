#[derive(Debug, PartialEq, Eq, Clone)]

pub enum DnsEncodeError{
    EmptyLabel,
    LabelTooLong(usize),    // maxim 63 (label e nume delimitat de . din adresa)
    DomainTooLong(usize),  // maxim 255 (nume complet de domeniu)
    InvalidCharacter(usize),
}