use std::str::FromStr;

pub fn parse_int(s: &str, base: u32) -> i64 {
    let s: String = s.chars().filter(|&c| c != '_').collect();
    i64::from_str_radix(&s, base).unwrap()
}

pub fn parse_float(s: &str) -> f64 {
    let s: String = s.chars().filter(|&c| c != '_').collect();
    f64::from_str(&s).unwrap()
}
