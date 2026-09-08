pub struct DnsPacketBuilder{
    id: u16,
    flags: u16,
    questions: Vec<DnsQuestion>,
    answers: Vec<DnsResourceRecord>,
    authorities: Vec<DnsResourceRecord>,
    additionals: Vec<DnsResourceRecord>,
}

impl DnsPacketBuilder{
    pub fn new(id: u16) -> Self {
        Self{
            id,
            flags:0,
            questions: vec![],
            answers: vec![],
            authorities: vec![],
            additionals: vec![],
        }
    }
    pub fn set_flags(mut self, flags: u16) -> Self{
        self.flags = flags;
        self
    }
    pub fn add_question(mut self, question: DnsQuestion) ->Self{
        self.questions.push(question);
        self
    }
    pub fn add_answer(mut self, answer: DnsResourceRecord) ->Self{
        self.answers.push(answer);
        self
    }
    pub fn add_authority(mut self, authority: DnsResourceRecord) ->Self{
        self.authorities.push(authority);
        self
    }
    pub fn add_aditional(mut self, data: DnsResourceRecord) ->Self{
        self.additionals.push(data);
        self
    }
    pub fn build(self) -> DnsPacket{
        let header = DnsHeader{
            id: self.id,
            flags: self.flags,
            qdcount: self.qdcount,
            ancount: self.ancount,
            nscount: self.nscount,
            arcount: self.arcount,
        };
        DnsPacket{
            header,
            questions: self.questions,
            answers: self.answers,
            authorities: self.authorities,
            additionals: self.additionals,
        }
    }
}