use regex::Regex;
use json::object;
use std::io;  
use std::time::{UNIX_EPOCH, SystemTime};

fn log_entry(message: String, severity: &str) -> json::JsonValue {
    let duration_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards");
    return object! {
        severity: severity,
        message: message.trim(),
        timestamp: {
            seconds: duration_since_epoch.as_secs(),
            nanos: duration_since_epoch.subsec_nanos()
        }
    };
}

fn parse(s: String, re: &Regex) -> json::JsonValue {            
    // if it's json, assume it's good and pass it on
    match json::parse(&s) {
        Ok(json) => {return json;}
        Err(_) => {
            // if it's not json, try to parse the severity out of it and turn it into json
            match re.captures_iter(&s).next() {
                Some(cap) => {
                    let severity = cap[1].to_uppercase();
                    return log_entry(s, &severity);
                }
                None => { // if everything else fails, assume severity is Info and put everything into the message field
                    return log_entry(s, "INFO");
                }
            } 
        }
    }
}

fn main() {    
    let re = Regex::new(r"(?i)\b(fatal|error|err|warning|warn|info|debug|trace)\b").expect("failed to compile regex");
    loop {
        let mut input_line = String::new();
        let bytes_read = io::stdin().read_line(&mut input_line).expect("can't read from stdin, exiting");
        if bytes_read == 0 { // reached EOF
            break;
        }
        let input_line = input_line;
        let log_entry = parse(input_line, &re);
        println!("{}", log_entry);

    }
}
