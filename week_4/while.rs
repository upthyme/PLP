fn main() {
    let mut count = 0;

    while count < 60 {
        println!("Counting down to 60! {} remain", 60 - count);
        println!("Count: {}", count);
        if count == 30 {
            println!("Halfway there!");  
        }
        if count % 2 == 0 {
            println!("I love a good even number.");
        }
        if count == 6 {
            println!("My favorite number!");
        }
        count += 1;
    } 
}