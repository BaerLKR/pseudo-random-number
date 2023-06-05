fn main() {
    println!("{}", pseudorandom(100))
}
fn pseudorandom(max: i32) -> i32 {
    let out = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(n) => format!(
            "{:.0}",
            max as f64
                * (32957239857.0 * n.as_micros() as f64 * 2.0 * std::f64::consts::PI)
                    .sin()
                    .abs()
        )
        .parse()
        .expect("Error making the random number to a number"),
        Err(e) => panic!("SystemTime before UNIX EPOCH! ({e})"),
    };
    out
}
