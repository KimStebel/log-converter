use regex::Regex;
use json::object;
use std::io;  
use std::time::{UNIX_EPOCH, SystemTime};

fn log_entry(message: &str, severity: &str) -> json::JsonValue {
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

fn parse(s: &str, re: &Regex) -> json::JsonValue {            
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

fn regex() -> Regex {
    return Regex::new(r"(?i)\b(fatal|error|err|warning|warn|info|debug|trace)\b").expect("failed to compile regex");
}

fn main() {    
    let re = regex();
    loop {
        let mut input_line = String::new();
        let bytes_read = io::stdin().read_line(&mut input_line).expect("can't read from stdin, exiting");
        if bytes_read == 0 { // reached EOF
            break;
        }
        let log_entry = parse(&input_line, &re);
        println!("{}", log_entry);

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let re = regex();
    
        assert_eq!(parse("test", &re)["message"], "test");
        assert_eq!(parse("debug test", &re)["severity"], "DEBUG");
        assert_eq!(parse("debugging is fun", &re)["severity"], "INFO"); // if the severity word isn't surrounded by word boundaries, it doesn't count, so default to info
        assert_eq!(parse("oh no there was an error!!", &re)["severity"], "ERROR");
        assert_eq!(parse("this is your last warning, dude!", &re)["severity"], "WARNING");
        assert_eq!(parse("WARN uh oh", &re)["severity"], "WARN");
        assert_eq!(parse("a trace of lsd is not an error", &re)["severity"], "TRACE"); // take the first occurance of a severity word
             
    }
}
