const PI: f64 = 3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679;

pub fn factorial(num: u32) -> u128 {
    let mut fact: u128 = 1;
    for i in 1..=num as u128 {
        fact *= i;
    }
    fact
}

pub fn chudnovsky_simplified(max_q: u32) -> f64 {
    let c = 426880_f64 * 10005_f64.sqrt();
    let mut sum: i128 = 0;
    let mut approximation: f64 = 0.;
    for q in 0..=max_q {
        sum += (factorial(6*q) * (545140134*q as u128 + 13591409)) as i128 / ((factorial(3*q) * factorial(q).pow(3)) as i128 * 262537412640768000_i64.pow(q) as i128);
        approximation = c / sum as f64;
        println!("Approximation at q={:?}: {:.52}", q, approximation);
        let error: f64 = approximation - PI;
        println!("Error: {:.52}", error);
    }
    
    approximation
}

#[test]
fn factorial_of_25() {
   assert_eq!(15511210043330985984000000, factorial(25));
}

#[test]
fn factorial_of_0() {
   assert_eq!(1, factorial(0));
}