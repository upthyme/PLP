/* Assignment 2: Data types and naming conventions

These are non-discussion questions asked in the prelude. 

> Can you add ints and floats? Is the resulting variable an int (narrowed) or float (widened)?

Yes. You need to define what to convert. Are you going to convert the int to float, or the float to int? If you convert the float to an int, it will lose its decimals.

You can cast with the "as" keyword.

Thre will always be loss with some data type conversions.
- Float to an integer will round float to 0
- Integer to float will produce closest possible float with rounding
- Large into to small int will truncate

> Can you put different data types in the same array or list?

No! Arrays must have only one data type. Look into structs, tuples, or enums if you want to store different types of data. 

> Can one data type be converted to another? (int to float, string to int)

Not easily. It really depends if someone has written a implementation for it, ie. "impl Add<SomeDataType> for AnotherDataType"

Type conversion is actually an incredibly deep topic that I'm still learning. 

Using "as" is explicit and works on primitive types. It cannot work on Strings or vectors, but it can work on integers, bool, str, and pointers.
Note that str and String are different types.

The two main traits used for conversion are "from" and "into". You can use "from" to create special types with a struct, and custom how you'd want
a conversion to go. You can use "into" for basically... opposite "from". Instead of A to B, you are defining what turns B INTO A. 

Usually not. Rust doesn't want to make the decision for you on how to convert. Instead of making a risky decision, it will raise a warning.

For some data types, Rust does feel confident and has an implementation. For instance, you can convert a string literal to a String with the `to_string()` method. 

Note: Avoid sentinel values
*/
use std::collections::HashMap;

fn main() {
    // Common Rust built-in data types:
    // boolean 
    // integers: i8, i16, i32, i64, u8, u16, u32, u64
    // floats: f32, f64
    // characters
    
    // bool
    // can be true or false
    // bools are also often used against a Option type that represents an optional type
    // an Option is either Some (if it has a value) or None (if it does not)
    // ... where's null? There's isn't a null reference. Pointers must always have a valid location in memory.
    let has_license: bool = false;

    // integers are either signed (i) or unsigned (u)
    // numbers represent size, a u8 integer is a 8 bit, unsigned integer 
    // isize and usize are dependent on the computer's implementation
    // Ex. a i16 integer has a range of 
    let some_int = 5; // note: default type of Rust integer is i32 
    let some_small_int: i8 = 5;   
    let some_medium_int: i64 = 2;
    let some_big_int: u16 = 2;
    let some_very_big_int: u64 = 2;
    let some_isize_int: isize = 1;
    let some_usize_int: usize = 1;

    let some_float = 2.2; // note: default type of Rust float is f64

    // char represents a single Unicode scalar value
    // notice the single quote ' which denotes that its a char
    let some_char = 'A'; 

    // arrays are fixed-size and of only one type
    let array: [u8; 3] = [1, 2, 3];

    // slices 
    let slice = &array[0..3]; // This will select the elements at index 0, 1, and 2. The first number is inclusive and second number is exclusive.

    // tuple 
    // collection that can contain multiple different types of data 
    let some_tuple = (1u8, true, 'a');

    // hashmap is the equivalent of a dictionary in Rust
    let mut music = HashMap::new();
    // values are stored by a key
    // keys needs to implement the Eq and Hash traits
    // by default, keys can be booleans, integers, and strings
    println!("Is hashmap empty: {}", music.is_empty());
    println!("Inserting data into the hashmap");
    music.insert("David Bowie", "The Man Who Sold the World");
    music.insert("Cat Stevens", "Trouble");
    music.insert("The Julie Ruin", "Hit Reset");
    music.insert("Digable Planets", "Where I'm From");
    println!("Is hashmap empty: {}", music.is_empty());

    // Rust never performs arithmetic operations on variables that belong to non-identical data types or offshoots of identical data types!
    // Rust discourages mixing of types
    // You *cannot* even add integers of different types without some kind of conversion (ie. a i32 integer with a i64 integer)

    // Adding ints and floats 
    println!("Int is: {}", some_int);
    println!("Float is: {}", some_float);

    // Casting with `as` or `From`
    println!("Trying to add int and float: {}", some_int + some_float as i32);
    println!("Trying to add int and float: {}", some_int as f64 + some_float);
}
