use super::Record;

pub fn parse_answers(answers: &[Record]) {
    answers
        .iter()
        .map(|answer| answer.rdata().as_ns())
        .for_each(|ns| println!("[*] Nameserver {}", ns.unwrap().to_string()));
}
