mod a;
mod ns;
mod soa;
mod txt;
mod mx;
mod aaaa;
mod cname;

use trust_dns_client::rr::{Record, RecordType};


pub fn parse_answers(answers: &[Record], query_type: RecordType) {
    match query_type {
        RecordType::A => a::parse_answers(answers),
        RecordType::NS => ns::parse_answers(answers),
        RecordType::SOA => soa::parse_answers(answers),
        RecordType::TXT => txt::parse_answers(answers),
        RecordType::MX => mx::parse_answers(answers),
        RecordType::AAAA => aaaa::parse_answers(answers),
        RecordType::CNAME => cname::parse_answers(answers),
        _ => println!("[*] {} Not implemented", query_type.to_string())
    };
}
