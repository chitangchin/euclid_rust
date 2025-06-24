fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp: u32 = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let x: u32 = 10;
    let y: u32 = 24;
    println!("The GCD of {} and {} is {}", x, y, gcd(x,y)); //The GCD of 10 and 24 is 2
    
    let a: u32 = 10;
    let b: u32 = 25;
    println!("The GCD of {} and {} is {}", a, b, gcd(a,b)); //The GCD of 10 and 25 is 5
}
