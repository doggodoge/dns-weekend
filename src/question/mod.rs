use std::io::{Cursor, Read};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

mod qclass;
mod qtype;

pub use qclass::QClass;
pub use qtype::QType;

pub struct Question {
    pub name: String,
    pub r#type: u16,
    pub class: u16,
}

impl Question {
    pub fn new(qname: &str, qtype: QType, qclass: QClass) -> Self {
        Question {
            name: String::from(qname),
            r#type: qtype as u16,
            class: qclass as u16,
        }
    }

    fn serialize_name(domain: &str) -> Vec<u8> {
        let mut result = vec![];
        for label in domain.split(".") {
            result.push(label.len() as u8);
            result.extend_from_slice(label.as_bytes());
        }
        result.push(0);
        return result;
    }

    // TODO: This won't work yet, add handling for DNS name compression
    fn deserialize_name(name_cursor: &mut Cursor<Vec<u8>>) -> String {
        let mut labels = vec![];

        loop {
            let length = name_cursor.read_u8().unwrap();

            if length == 0 {
                break;
            }

            let mut label = vec![0; length as usize];
            name_cursor.read_exact(&mut label).unwrap();

            labels.push(String::from_utf8_lossy(label.as_slice()).into_owned());
        }

        return labels.join(".");
    }
}

impl From<Question> for Vec<u8> {
    fn from(value: Question) -> Self {
        let mut buf = vec![];
        buf.extend(Question::serialize_name(&value.name));
        buf.write_u16::<BigEndian>(value.r#type).unwrap();
        buf.write_u16::<BigEndian>(value.class).unwrap();
        return buf;
    }
}

impl From<Vec<u8>> for Question {
    fn from(value: Vec<u8>) -> Self {
        let mut cursor = Cursor::new(value);
        Question {
            name: Question::deserialize_name(&mut cursor),
            r#type: cursor.read_u16::<BigEndian>().unwrap(),
            class: cursor.read_u16::<BigEndian>().unwrap(),
        }
    }
}
