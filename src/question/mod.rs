use byteorder::{BigEndian, WriteBytesExt};

mod qclass;
mod qtype;

pub use qclass::QClass;
pub use qtype::QType;

#[repr(packed)]
pub struct Question {
    pub name: Vec<u8>,
    pub r#type: u16,
    pub class: u16,
}

impl Question {
    pub fn new(qname: &str, qtype: QType, qclass: QClass) -> Self {
        Question {
            name: Question::serialize_qname(qname),
            r#type: qtype as u16,
            class: qclass as u16,
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
        buf.extend(value.name);
        buf.write_u16::<BigEndian>(value.r#type).unwrap();
        buf.write_u16::<BigEndian>(value.class).unwrap();
        return buf;
    }
}
