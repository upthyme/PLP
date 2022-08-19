fn main() {
    // Trying to do `x = "5" + 6` 
    let x = "5".to_string().parse::<i32>().unwrap() + 6;
    println!("{}", x);
}