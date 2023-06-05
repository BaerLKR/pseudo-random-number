fn main() {
    use std::time::{SystemTime, UNIX_EPOCH};

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => {
            let n = n.as_secs();
            let n = n as f64;
            let n = n.sin().cos();
            println!("{n}");
        }
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
