// 21/08/2019
//
// Implement pow(x, n), which calculates x raised to the power n, (x^n). 
//
// Example:
// Input: x = 2, n = 5
// Output: 32
//
// Input: x = 2, n = -2
// Output: 0.25
//
// Input: x = 2.5, n = 3
// Output: 15.625

fn pow(x: f64, mut n: i32) -> f64 {
    let is_neg = n < 0;
    n = n.abs();
    let mut p: f64 = x;   
    let mut result: f64 = 1.0;

    while n != 0 {
        if n % 2 == 1{
            result *= p; 
        }

        n /= 2;
        p = p * p;
    }

    return if is_neg { 1.0 / result } else { result };
}

fn main() {
    println!("Expression: 2^5, Result: {}", pow(2.0, 5));
    println!("Expression: 2^-2, Result: {}", pow(2.0, -2));
    println!("Expression: 2.5^3, Result: {}", pow(2.5, 3));
} 
