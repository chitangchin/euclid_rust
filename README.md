# Euclid Algorithm

Finding the Greatest common denominator of two numbers

## Example:

Given two unsigned integers of 32 bytes:
let a: u32 = 25522;
let b: u32 = 1024;

Summary: we will keep taking the result of a modulo b, which is the remainder when dividing a by b, until b is equal to 0. Euclid's Algorithm proves that performing this operation, you will reach the GCD of any two numbers

Time complexity: O(log min(a,b))
Space complexity: O(1)

### How the algorithm works:

Within our GCD function, we will have to iterate until b is equal to 0, lets create a while loop

```
while b != 0 {

}
```

Within our while loop we will do the following:

1. Initialize a temp variable to store the initial b
let temp: u32 = b;

2. set b to the remainder of a / b: in otherwords a % b
b = a % b;

3. Now that we've stored the remainder of a and b to b, we want to now update a to contain the value of b. Remember we stored the b value in the temp variable
a = temp;

```
while b != 0 {
    let temp: u32 = b;
    b = a % b;
    a = temp;
}
```

4. Your function should have a return value for a unsigned 32 byte integer, and we can return the expression of a which will be the greatest common denominator of the two starting values:

Full Code:
```
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp: u32 = b;
        b = a % b;
        a = temp;
    }
    a
}
```

5. For a visual of how the variables will update for the given values above, please use a debugger. For Rust you can install two extensions called **rust-analyzer** and **CodeLLDB**.

Once you install CodeLLDB, you want to navigate to the src/main.rs file and click on create config file under the run and debug tab.

Here you can run into at tab while loop to see how the value is updated.

