fn main(){
    let rating = 80;
    match rating {
        // Spectacular singular value job!
        100 => println!("Spectacular"),
        // Impressive job for several values
        99 | 98 | 97 | 96 | 95 => println!("Impressive"),
        // Good work for value ranges
        80..=94 => println!("Great"),
        50..=80 => println!("Okay"),
        0..=50 => println!("No comment"),

        // The default
        _ => println!("No idea."),
    };
}