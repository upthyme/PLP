fn main() {
    let mut even_sum = 0;
    let mut odd_sum = 0;
    let mut total = 0; // what happens if you change this to a 0.0? Rust will see it as a float and theres no trait for += with a i32
    for i in 1..101 { // add up numbers 1 to 100
        if i % 2 == 0 {
            even_sum += i;
            println!("Evens: {}", even_sum);
        } else {
            odd_sum += i;
            println!("Odds: {}",odd_sum);
        }
        total += i;
    }
    println!("\nTotal: {}", total);
    println!("Total Even: {}", even_sum);
    println!("Total Odd: {}", odd_sum);
}