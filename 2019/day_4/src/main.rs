use std::time::Instant;


fn main() {
    let now = Instant::now();
    let mut j = 0;
    for i in 402328..864247 + 1 {
        j = i;
    }
    println!("Jee {}", j);
    println!("Time elapsed: {}ms", now.elapsed().as_millis());

}
