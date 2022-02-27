use regex::Regex as R;

fn main() {
    let line = "2022-01-01 12:42:34.000Z INFO some message here";
    let re = R::new(r"^(\d{4})-(\d{2})-(\d{2})").unwrap();

    let cap = re.captures_iter(line).next().unwrap();
    println!("Month: {} Day: {} Year: {}", &cap[2], &cap[3], &cap[1]);
    println!("{}", re.is_match(line));
    println!("end");
}
