// Write a function that takes in two numbers, multiplies them, and returns the output

fn multiply_numbers(x: i32, y:i32) -> i32 {
    x * y
}

fn main(){
    let num1 = 23;
    let num2 = 65;
    let result = multiply_numbers(num1, num2);
    println!("{}", result);

}