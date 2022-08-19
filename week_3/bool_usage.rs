fn main(){
    let alive = true; 

    // assert
    assert!(alive); 
    // example of a cast
    assert_eq!(alive as i64,1);

    // if condition
    if alive {
        println!("Pay your taxes!");
    } else {
        println!("...");
    }

    // match pattern
    match alive {
        true => println!("Keep on truckin'"),
        false => println!("..."),
    }
}