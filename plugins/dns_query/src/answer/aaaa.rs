use super::Record;

pub fn parse_answers(answers: &[Record]) {
    answers
        .iter()
        .map(|answer| answer.rdata().as_aaaa())
        .for_each(|ipv6| println!("[*] IPv6 Address {}", ipv6.unwrap()));
}
