
fn main() {
    let mut x = 1;
    'outer: loop {
        println!("Enter: Outer");
        println!("{}", x);

        'inner: loop {
            println!("Enter: Inner");
            let x = 10;
            println!("{}", x);
            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }
        let x = 420;
        println!("Never have I ever been executed!");
        println!("{}", x);
    }

    println!("Enter: Outer");
    println!("{}", x);
}