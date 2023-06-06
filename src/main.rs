fn main() {
    println!("{}", pseudorandom([0, 200]))
}
fn pseudorandom(minmax: [i32; 2]) -> i32 {
    let offs = (minmax[0] as f64 + minmax[1] as f64) * 0.5;
    let add = minmax[0] as f64 - offs;
    let out = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(n) => format!(
            "{:.0}",
            add * (32957239857.0 * n.as_micros() as f64 * 2.0 * std::f64::consts::PI).sin() + offs
        )
        .parse()
        .expect("Error making the random number to a number"),
        Err(e) => panic!("SystemTime before UNIX EPOCH! ({e})"),
    };
    out
}
