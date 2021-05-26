use super::Record;

pub fn parse_answers(answers: &[Record]) {
    answers
        .iter()
        .map(|answer| answer.rdata().as_mx())
        .for_each(|mx| {
            let mx = mx.unwrap();

            println!("[*] {} preference {}", mx.exchange(), mx.preference());
        });
}
