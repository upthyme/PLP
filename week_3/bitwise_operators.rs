fn main() {
    let a:u8 = 6; // 110     
    let b:u8 = 1; // 001    
    let c:u8 = 3; // 011
    let d:u8 = 255; // 11111111

    println!("a: {}\nb: {}\nc: {}\nd: {}\n", a,b,c,d);
    // TODO: figure out binary repr here

    // Examples of bitwise operators on the numbers

    // AND
    println!("(a & b) = {} ",a & b); // 000, 0 
    println!("(a & c) = {} ",a & c); // 010, 2 
 
    // OR
    println!("(a | b) = {} ",a | b) ; // 111, 7
    println!("(a | c) = {} ",a | c) ; // 111, 7
    println!("(b | c) = {} ",b | c) ; // 011, 3
 
    // XOR
    println!("(a ^ a) = {} ",a ^ a); // 0  
    println!("(a ^ c) = {} ",a ^ c); // 101, 5
 
    // NOT
    // Kind of like a reversal of bits
    // Why so large? Because of unsigned.
    println!("(!b) = {} ",!b); // 11111110, 254 
    println!("(!c) = {} ",!c); // 252
    println!("(!d) = {} ",!d); // 00000000, 0
 
    // Shift left
    // Equivalent to doubling 
    println!("(a << 1) = {}",a << 1); // 1100, 12
    println!("(b << 1) = {}",b << 1); // 010, 2
 
    // Shift right 
    println!("(a >> 1) = {}",a >> 1); // 011, 3

 }