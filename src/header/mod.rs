use byteorder::{BigEndian, WriteBytesExt};

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
        buffer.write_u16::<BigEndian>(value.num_authorities).unwrap();
        buffer.write_u16::<BigEndian>(value.num_additionals).unwrap();
        return buffer;
    }
}
