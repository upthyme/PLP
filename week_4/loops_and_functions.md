# Assignment 4: Loops and Functions

## Discussion Questions

**1. Does your language include multiple types of loops (while, do/while, for, foreach)? If so, what are they and how do they differ from each other?**

Yes, multiple styles of loops are supported. 
- for
- loop
- outer/inner loop
- while

The `for` loop is used like in other languages. To use a `for` loop, you must specify an iterator. This can be a range, `a..b`, or from a collection.

The `loop` keyword indicates an *infinite* loop. You must have a `break` statement to exit the loop, otherwise it will go on forever.

The `outer: loop` and the `inner: loop` represent nested loops. Note that there is a label in the first part. When you want to break/continue in these loops, you must specify the label. 

TODO: Do you have to use the outer/inner loops?

The `while` loop runs a loop while a condition is true. 

**2. What is the syntax for declaring a function in your language?**

A function in Rust is written with `fn` keyword with the function name and a set of parentheses. Note that every Rust program must have a main function. This is `fn main() {...}`. 

In the function signature, you need to declare the types. The parameters will be first, and then there's an ASCII arrow (`->`) pointing to the return type. 

Note: Python 3's type hinting is similar.

Note that a Rust function doesn't necessarily have to have a explicit return value. The last expression in the function is *implicitly* considered to be the return value. 

**3. Are there any rules about where the function has to be placed in your code file so that it can run?**

You can place functions anywhere in the file. Usually, I put functions outside of main. 

**4. Does your language support recursive functions?**

Yes. See code. However, Rust style favors iterators.

**5. Can functions in your language accept multiple parameters? Can they be of different data types?**

Yes, the functions can accept multiple parameters of different data types. Note that this must be declared in the signature because Rust is statically typed. 

**6. Can functions in your language return multiple values at the same time? How is that implemented? If not, are there ways around that problem? What are they?**

Yes, you can return multiple values. You can also return a tuple with more than one value and of different types.

``` rust 
fn return_tuple() -> (u32, bool, String){
    (10,false,"essen")
}
```

Note that (1) there is no return and that (2) there is no semicolon on the tuple line. This is because the line is an expression. If you add a semicolon, the line will be interpreted as a statement and nothing will be returned by the function! 

**7. Is your language pass-by reference or value? Check your code against outside sources in case there is anything tricky going on (like in Perl).**

**8. Are there any other aspects of functions in your language that aren't specifically asked about here, but that are important to know in order to write one? What are they?**

- You can change traits that are used for operators. For example, `+=` is the `AddAssign` trait and can be changed.
- Its important to note the difference between statements and expressions.
- You can return a value from loops by an expression after the `break` statement

### References 
https://doc.rust-lang.org/rust-by-example/flow_control/loop.html
https://www.educative.io/answers/how-to-write-functions-in-rust
https://stackoverflow.com/questions/18479648/does-rust-have-support-for-functions-that-return-multiple-values 