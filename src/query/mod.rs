use rand::Rng;

use crate::{
    header::Header,
    question::{QClass, QType, Question},
};

pub struct Query;

impl Query {
    pub fn build_query(domain_name: &str, record_type: QType) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let id = rng.gen_range(0..65535);
        let recursion_desired = 1 << 8 as u8;

        let header = Header {
            id,
            flags: recursion_desired,
            num_questions: 1,
            num_additionals: 0,
            num_answers: 0,
            num_authorities: 0,
        };
        let question = Question::new(domain_name, record_type, QClass::IN);

        let mut buf = vec![];
        buf.extend(Vec::from(header));
        buf.extend(Vec::from(question));
        return buf;
    }
}
