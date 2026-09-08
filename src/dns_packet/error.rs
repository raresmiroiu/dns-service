#[derive(Debug, PartialEq, Eq, Clone)]

pub enum DnsEncodeError{
    EmptyLabel,
    LabelTooLong(usize),    // maxim 63 (label e nume delimitat de . din adresa)
    DomainTooLong(usize),  // maxim 255 (nume complet de domeniu)
    InvalidCharacter(usize),
}

impl std::fmt::Display for DnsEncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsEncodeError::EmptyLabel => {
                write!(f, "domain name contains an empty label (e.g. \"a..b\")")
            }
            DnsEncodeError::LabelTooLong(len) => {
                write!(f, "label is {len} bytes long, maximum is 63")
            }
            DnsEncodeError::DomainTooLong(len) => {
                write!(f, "encoded domain name is {len} bytes long, maximum is 255")
            }
            DnsEncodeError::InvalidCharacter(pos) => {
                write!(f, "invalid character in domain name at byte offset {pos}")
            }
        }
    }
}

impl std::error::Error for DnsEncodeError {}