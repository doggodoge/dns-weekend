use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[repr(packed)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Header {
    pub id: u16,
    pub flags: u16,
    pub num_questions: u16,
    pub num_answers: u16,
    pub num_authorities: u16,
    pub num_additionals: u16,
}

impl From<Header> for Vec<u8> {
    fn from(value: Header) -> Self {
        let mut buffer = vec![];
        buffer.write_u16::<BigEndian>(value.id).unwrap();
        buffer.write_u16::<BigEndian>(value.flags).unwrap();
        buffer.write_u16::<BigEndian>(value.num_questions).unwrap();
        buffer.write_u16::<BigEndian>(value.num_answers).unwrap();
        buffer
            .write_u16::<BigEndian>(value.num_authorities)
            .unwrap();
        buffer
            .write_u16::<BigEndian>(value.num_additionals)
            .unwrap();
        return buffer;
    }
}

impl From<Vec<u8>> for Header {
    fn from(value: Vec<u8>) -> Self {
        let mut cursor = Cursor::new(value);

        return Header {
            id: cursor.read_u16::<BigEndian>().unwrap(),
            flags: cursor.read_u16::<BigEndian>().unwrap(),
            num_questions: cursor.read_u16::<BigEndian>().unwrap(),
            num_answers: cursor.read_u16::<BigEndian>().unwrap(),
            num_authorities: cursor.read_u16::<BigEndian>().unwrap(),
            num_additionals: cursor.read_u16::<BigEndian>().unwrap(),
        };
    }
}
