// Write a function that takes in a string (or your language's equivalent) and splits it into two strings, then returns both strings
//
// This is actually really tough for me. 
//
// Used the Vector split string solution from Stack Overflow
// "How do I split a string in Rust?", StackOverflow https://stackoverflow.com/questions/26643688/how-do-i-split-a-string-in-rust (Accessed August 20, 2022)

fn split_string(s: &String) -> (String, String) {
    let splitted_string: Vec<&str> = s.split_whitespace().collect();
    (splitted_string[0].to_string(), splitted_string[1].to_string())
}

fn main(){
    let full_string = "Hello world".to_string();
    println!("{}",full_string);

    // split into two string
    let (string_1, string_2) = split_string(&full_string);

    println!("First part of String: {}", string_1);
    println!("Second part of String: {}", string_2);

}