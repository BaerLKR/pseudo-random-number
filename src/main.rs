fn main() {
    use std::time::{SystemTime, UNIX_EPOCH};

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            let n = n.as_micros();
            println!("{n}");
            let n = n as f64;
            let n = n.tan().sin();
            let n = n * 100000000.0;
            let n = n.abs();
            let n = format!("{:.0}", n);
            println!("{n}");
        }
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
