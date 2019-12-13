use std::time::Instant;
use fancy_regex::Regex;

fn main() {
    let now = Instant::now();

    let mut j = 0;
    let re1 = Regex::new(r"^(\w)\1+$").unwrap();
    let re2 = Regex::new(r"^(?=\d{6}$)1*2*3*4*5*6*7*8*9*0*$").unwrap();
    for i in 402328..864247 + 1 {
        if re1.is_match(&i.to_string()).is_ok() && re2.is_match(&i.to_string()).is_ok() {
            j += 1
        }
    }
    println!("1: {}", j);

    println!("Time elapsed: {}ms", now.elapsed().as_millis());

}
