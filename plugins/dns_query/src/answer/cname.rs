use super::Record;

pub fn parse_answers(answers: &[Record]) {
    answers
        .iter()
        .map(|answer| answer.rdata().as_cname())
        .for_each(|cname| println!("[*] Nameserver {}", cname.unwrap().to_string()))
}
