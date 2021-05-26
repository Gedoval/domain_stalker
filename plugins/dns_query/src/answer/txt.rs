use super::Record;

pub fn parse_answers(answers: &[Record]) {
    answers
        .iter()
        .map(|answer| answer.rdata().as_txt())
        .for_each(|txt| {
            let txt = txt.unwrap();
            println!("[*] {}", txt.to_string());
        });
}
