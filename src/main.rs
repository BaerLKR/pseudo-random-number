fn main() {
    println!("{}", pseudorandom(100))
}
const PI: f64 = std::f64::consts::PI;
fn pseudorandom(max: i32) -> i32 {
    let out: i32;
    use std::time::{SystemTime, UNIX_EPOCH};
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            let n = n.as_micros() as f64;
            let n = custom_sin(n, 3428975112.0, max);
            out = n.trim().parse().unwrap();
        }
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
    out
}
fn custom_sin(x: f64, frequency: f64, max: i32) -> String {
    let c = max as f64 * (frequency * x * 2.0 * PI).sin();
    format!("{:.0}", c.abs())
}
