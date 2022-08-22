// Call your functions and save the results of the function calls in variables.

fn is_even(x: u32) -> bool {
    x % 2 == 0 
}

// we can't generate an array because its fixed size
// need to use vector
fn generate_vector() ->  Vec<u32> {
    let mut vec = Vec::with_capacity(10);
    for i in 0..100 {
        if !is_even(i){
            vec.push(i);
        }
    }
    vec
}

fn main() {
    let number = 10; 
    let is_number_even = is_even(number);
    println!("Number: {}", number);
    println!("Is it even? {}", is_number_even);

    let num_vector = generate_vector();
    println!("{:?}", num_vector);
}