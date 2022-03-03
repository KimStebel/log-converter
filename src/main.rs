use regex::Regex;
use json::object;
use std::io;  

fn parse(s: String, re: &Regex) -> json::JsonValue {            
    // if it's json, assume it's good and pass it on
    match json::parse(&s) {
        Ok(json) => {return json;}
        Err(_) => {
            // if it's not json, try to parse the severity out of it and turn it into json
            match re.captures_iter(&s).next() {
                Some(cap) => {
                    let severity = cap[1].to_uppercase();
                    return object! {
                        severity: severity,
                        message: s.trim(),
                        timestamp: {
                            seconds: 0,
                            nanos: 0     
                        }
                    };
                }
                None => { // if everything else fails, assume severity is Info and put everything into the message field
                    return object! {
                        severity: "INFO",
                        message: s.trim(),
                        timestamp: {
                            seconds: 0,
                            nanos: 0           
                        }
                    };
                }
            } 
        }
    }
}

fn main() {    
    let re = Regex::new(r"(?i)(fatal|error|err|warning|warn|info|debug|trace)").expect("failed to compile regex");
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
