use super::Record;

pub fn parse_answers(answers: &[Record]) {
    answers
        .iter()
        .map(|answer| answer.rdata().as_soa())
        .for_each(|soa| {
            let soa = soa.unwrap();

            println!("[+] Primary name {} ", soa.mname());
            println!("\t[-] Responsible {} ", soa.rname());
            println!("\t[-] Next refresh {} ", soa.refresh());
            println!("\t[-] Retry on failure {} ", soa.retry());
            println!("\t[-] Zone expiry {} ", soa.expire());
        });
}
