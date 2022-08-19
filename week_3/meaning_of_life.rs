fn main(){
    let the_meaning_of_life_the_universe_and_everything = 42;
    assert_eq!(the_meaning_of_life_the_universe_and_everything, 42); // If it doesn't ... Dont panic! But it does generate a runtime error.

    if the_meaning_of_life_the_universe_and_everything == 42 { // evaluate to true
        println!("Thank you, deep thought!");
    } else {
        println!("This had better not be Slartibartfast messing with me...");
    }
}