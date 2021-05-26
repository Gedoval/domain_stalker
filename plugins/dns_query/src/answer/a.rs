use super::Record;

pub fn parse_answers(answers: &[Record]) {
    answers
        .iter()
        .map(|answer| answer.rdata().as_a())
        .for_each(|ip| println!("[*] IPv4 Address {}", ip.unwrap()));
}
