fn main() {
    println!("{}", pseudorandom())
}
const PI: f64 = std::f64::consts::PI;
fn pseudorandom() -> i32 {
    let out: i32;
    use std::time::{SystemTime, UNIX_EPOCH};
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            let n = n.as_micros() as f64;
            // let n = format!("{:.0}", n.tan().sin() * 100000000.0);
            let n = format!("{:.0}", custom_sin(n, 340763470.0) * 1000000000.0);
            out = n.trim().parse().unwrap();
        }
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
    out
}
fn custom_sin(x: f64, frequency: f64) -> f64 {
    (frequency * x * 2.0 * PI).sin()
}
