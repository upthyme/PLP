// Write a function that tests whether your language is pass-by-reference or pass-by-value.
// Using same box example: https://doc.rust-lang.org/rust-by-example/scope/move.html 

fn break_box(b: Box<i32>) {
    println!("Bye {}", b);
}

fn main(){
    let a = Box::new(5i32);
    println!("{}",a);
    break_box(a);
    // a no longer exists!
    // break_box took ownership of it and threw it away!
    //println!("{}",a);
}
