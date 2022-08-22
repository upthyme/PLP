// Write a recursive function (one that calculates a factorial is fine)

// factorial is calculated f(i-1)
fn factorial(x: u32) -> u32 {
    // Base case
    // 1! -> 1 
    if x == 1 {
        1
    // Recursion time
    // x! = (x-1) * x
    } else {
        x * factorial(x -1)
    }
}
fn main(){
    let num = 11;
    let result = factorial(num);
    println!("{}", result);
}