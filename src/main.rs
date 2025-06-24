fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp: u64 = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let x: u64 = 10;
    let y: u64 = 24;
    println!("The GCD of {} and {} is {}", x, y, gcd(x,y)); //The GCD of 10 and 24 is 2
    
    let a: u64 = 10;
    let b: u64 = 25;
    println!("The GCD of {} and {} is {}", a, b, gcd(a,b)); //The GCD of 10 and 25 is 5
}
