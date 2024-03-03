use std::io::{stdin, Read};

/*
Each iteration parses one of these:

Transaction {hash}
Status: Confirmed
Amount received: {number (sometimes scientific notation, i.e. 1.234E-5)} XCH
To address: {address}
Created at: {datetime, has whitespace between date and time}
*/
const TRANSACTION_HEADER: &str = "Transaction ";
const STATUS_HEADER: &str = "Status: ";
const AMOUNT_HEADER: &str = "Amount ";
const RECEIVE: &str = "received: ";
const SENT: &str = "sent: ";
const REWARDED: &str = "rewarded: ";
const XCH_FOOTER: &str = " XCH";
const ADDRESS_HEADER: &str = "To address: ";
const CREATED_AT_HEADER: &str = "Created at: ";

#[derive(Debug)]
enum Event {
    Received,
    Sent,
    Rewarded
}

fn main() {
    let content = {
        let mut c = String::new();
        stdin()
            .read_to_string(&mut c)
            .expect("failed to read stdin to string");
        c
    };

    println!("transaction_hash,status,event_type,amount_received,to_address,created_at");

    let mut lines = content.lines();
    while let Some(t_line) = lines.find(|s| s.contains(TRANSACTION_HEADER)) {
        let hash = {
            let hash_index = t_line.find(TRANSACTION_HEADER).unwrap() + TRANSACTION_HEADER.len();
            String::from(&t_line[hash_index..])
        };

        let status = {
            let s_line = lines.next().expect("unexpected EOF");
            if !s_line.starts_with(STATUS_HEADER) {
                panic!(
                    "\"Transaction\" line not followed by \"Status\" line: {}",
                    s_line
                )
            }
            let s_index = STATUS_HEADER.len();
            String::from(&s_line[s_index..])
        };

        let (event_type, amount) = {
            let a_line = lines.next().expect("unexpected EOF");
            if !a_line.starts_with(AMOUNT_HEADER) {
                panic!(
                    "\"Status\" line not followed by \"Amount\" line: {}",
                    a_line
                )
            }
            let event = if a_line.contains(RECEIVE) {
                Event::Received
            } else if a_line.contains(SENT) {
                Event::Sent
            } else if a_line.contains(REWARDED) {
                Event::Rewarded
            } else {
                panic!(
                    "\"Amount\" line contains unknown event: {}",
                    a_line
                )
            };
            let amount_prefix = match event {
                Event::Received => "",
                Event::Rewarded => "",
                Event::Sent => "-",
            };

            let a_start = match event {
                Event::Received => AMOUNT_HEADER.len() + RECEIVE.len(),
                Event::Sent => AMOUNT_HEADER.len() + SENT.len(),
                Event::Rewarded => AMOUNT_HEADER.len() + REWARDED.len(),
            };
            let a_end = a_line.len() - XCH_FOOTER.len();
            (event, format!("{}{}", amount_prefix, &a_line[a_start..a_end]))
        };

        let address = {
            let a_line = lines.next().expect("unexpected EOF");
            if !a_line.starts_with(ADDRESS_HEADER) {
                panic!(
                    "\"Amount\" line not followed by \"To address\" line: {}",
                    a_line
                )
            }
            let a_index = ADDRESS_HEADER.len();
            String::from(&a_line[a_index..])
        };

        let created_at = {
            let c_line = lines.next().expect("unexpected EOF");
            if !c_line.starts_with(CREATED_AT_HEADER) {
                panic!(
                    "\"Amount\" line not followed by \"Created at\" line: {}",
                    c_line
                );
            }
            let c_index = CREATED_AT_HEADER.len();
            String::from(&c_line[c_index..])
        };

        println!("{},{},{:?},{},{},{}", hash, status, event_type, amount, address, created_at)
    }
}
