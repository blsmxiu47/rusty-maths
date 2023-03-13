use std::time::Instant;

use rusty_maths::chudnovsky_simplified;

fn main() {
    // With this implementation of Chudnovsky method will panic at max_q >= 2 XD
    let max_q: u32 = 1;
    println!("Running Chudnovsky (simplified) approximation of Pi for {:?} iterations...", max_q + 1);

    let start = Instant::now();
    chudnovsky_simplified(max_q);
    let duration = start.elapsed();
    println!("Time elapsed in chudnovsky_simplified({:?}) is {:?}", max_q, duration);
}
