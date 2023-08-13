use byteorder::{BigEndian, WriteBytesExt};

mod qclass;
mod qtype;

pub use qclass::QClass;
pub use qtype::QType;

#[repr(packed)]
pub struct Question {
    pub qname: Vec<u8>,
    pub qtype: u16,
    pub qclass: u16,
}

impl Question {
    pub fn new(qname: &str, qtype: QType, qclass: QClass) -> Self {
        Question {
            qname: Question::serialize_qname(qname),
            qtype: qtype as u16,
            qclass: qclass as u16,
        }
    }

    fn serialize_qname(domain: &str) -> Vec<u8> {
        let mut result = vec![];
        for label in domain.split(".") {
            result.push(label.len() as u8);
            result.extend_from_slice(label.as_bytes());
        }
        result.push(0);
        return result;
    }
}

impl From<Question> for Vec<u8> {
    fn from(value: Question) -> Self {
        let mut buf = vec![];
        buf.extend(value.qname);
        buf.write_u16::<BigEndian>(value.qtype).unwrap();
        buf.write_u16::<BigEndian>(value.qclass).unwrap();
        return buf;
    }
}
