/* Assignment 2: Data types and naming conventions

These are non-discussion questions asked in the prelude. 

> Can you add ints and floats? Is the resulting variable an int (narrowed) or float (widened)?

Yes. You need to define what to convert. Are yo going to convert the int to float, or the float to int? If you convert the float to an int, it will lose its decimals.

You can cast with either "from" or "as"

> Can you put different data types in the same array or list?

No! Arrays must have only one data type. Look into structs or tuples if you want to store different types of data. 

> Can one data type be converted to another? (int to float, string to int)

Not easily. It really depends if someone has written a implementation for it, ie. "impl Add<SomeDataType> for AnotherDataType"

Usually not. Rust doesn't want to make the decision for you on how to convert. Instead of making a risky decision, it will raise a warning.

For some data types, Rust does feel confident and has an implementation. For instance, you can convert a string literal to a String with the `to_string()` method. 

Note: Avoid sentinel values
*/

fn main() {
    // Common Rust built-in data types:
    // boolean 
    // integers: i8, i16, i32, i64, u8, u16, u32, u64
    // floats: f32, f64
    // characters
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

    let some_char = 'A'; 

    // arrays are fixed-size and of one type
    let array: [u8; 3] = [1, 2, 3];

    // slices 
    let slice = &array[0..3]; // This will select the elements at index 0, 1, and 2. The first number is inclusive and second number is exclusive.
    println!("Int is: {}", some_int);
    println!("Float is: {}", some_float);
    
    // Rust never performs arithmetic operations on variables that belong to non-identical data types or offshoots of identical data types!
    // Rust discourages mixing of types
    // You *cannot* even add integers of different types without some kind of conversion (ie. a i32 integer with a i64 integer)

    // Casting with `as` or `From`
    println!("Trying to add int and float: {}", some_int + some_float as i32);
    println!("Trying to add int and float: {}", some_int as f64 + some_float);
}
