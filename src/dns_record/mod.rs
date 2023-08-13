use crate::question::{QType, QClass};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DNSRecord {
    pub name: Vec<u8>,
    pub r#type: u16,
    pub class: u16,
    pub ttl: u16,
    pub data: Vec<u8>,
}

impl DNSRecord {
    // pub fn new(name: &str, r#type: QType, class: QClass, ttl: u16, data: Vec<u8>) -> Self {
    //     DNSRecord {
    //         name: 
    //     }
    // }
}
