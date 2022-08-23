# Assignment 4: Loops and Functions

## Discussion Questions

**1. Does your language include multiple types of loops (while, do/while, for, foreach)? If so, what are they and how do they differ from each other?**

Yes, multiple styles of loops are supported. 
- for
- loop
- outer/inner loop
- while

The `for` loop is used like in other languages. To use a `for` loop, you must specify an iterator. This can be a range, `a..b`, or from a collection. For example, `for n in 1..10` will go over 1 to 9[^ForLoops]. See [simple_for.rs](simple_for.rs).

The `loop` keyword indicates an *infinite* loop. You must have a `break` statement to exit the loop, otherwise it will go on forever.

The `outer: loop` and the `inner: loop` represent nested loops. Note that there is a label in the first part. When you want to break/continue in these loops, you must specify the label. See [nested_loop.rs](nested_loop.rs).

The `while` loop runs a loop while a condition is true. See [while.rs](while.rs).

**2. What is the syntax for declaring a function in your language?**

A function in Rust is written with `fn` keyword with the function name and a set of parentheses. Note that every Rust program must have a main function. This is `fn main() {...}`. 

In the function signature, you need to declare the types. The parameters will be first, and then there's an ASCII arrow (`->`) pointing to the return type. 

Below is an example function signature. It takes in a u8 integer, and returns a String.

``` rust 
fn foo(x: u8) -> String {
    if x == 0 {
        "foo".to_string()
    }
    "bar".to_string()
}
```

Notes: 
- Python 3's type hinting is similar to Rust's type annotations. 
- Rust functions don't necessarily have to have a explicit return value. The last expression in the function is *implicitly* considered to be the return value. 

**3. Are there any rules about where the function has to be placed in your code file so that it can run?**

You can place functions anywhere in the file. Usually, I put functions outside of `main`. 

**4. Does your language support recursive functions?**

Yes. See [recursive_func.rs](recursive_func.rs) for a basic factorial implementation. 

In general, Rust style favors iterators.

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

Pass by value means that when passing an argument into the function, the value gets _copied_ into the new function. The value in the "calling function" and the "called function" are separate [^Ryan].
Pass by reference means that we pass in a variable and its effect in the function is seen by the callee. The identity is the same.

Rust is a pass by value language. 

Rust will literally `move` values. If we pass in the literal value into a function, we will actually move it into the function. What this `move` means is a change of ownership. The code below
shows that the variable `xpaint` exists until it is taken ownership by the `spin` function. Once that function exits, the variable no longer exists in any scope.

``` rust
fn spin (paint: Paint) {
    ...
}

// x exists
let xpaint = Paint(100,20,30);
println!("{}",xpaint);
println!("{}",spin(xpaint)); // x doesn't exist anymore when the function block ends!

// following code will err out: 
// println!("{}",xpaint);
```

The way to preserve ownership of variables in our scope is to instead pass in a reference to the variable. When we pass in a reference, we only _borrow_ the value. This means that after the function ends, the 
value still exists in its original scope. If we used to below code on `xpaint`, we would borrow the value with the `&` and the value of `xpaint` wouldn't change. 

``` rust 
fn spin (paint: &Paint) {
    ...
}
```

Even though we often pass in references to functions, the references act as values that are passed in by value to the function. A reference of a variable `number_of_carrots` would be `&number_of_carrots` and point to an address we can follow to access data at that address.

Note that if we wanted to change the value of variable `xpaint`, we could pass in a mutable reference like `mut &xpaint`.

It is confusing that we are _pass by value_ even when we are literally passing in _references_ in Rust. Try to remember that Rust's main goal is memory safety. To achieve that, there is a concept of ownership. Each value has a owner, there is only one owner at a time, and when the owner goes out of scope - the value is dropped [^Ownership].

**8. Are there any other aspects of functions in your language that aren't specifically asked about here, but that are important to know in order to write one? What are they?**

- Use a tuple to return multiple values from a function (esp. different types) [^MultipleValues]
- Scope is important. You can `move` variables without thinking if you pass them into functions without references 
- There mutable and immutable references.
    - Mutable references can change the variable
    - Immutable references do not change the variable
    - You can only have _one_ mutable reference at a time. If you have a mutable reference, you cannot have any immutable references.
    - You can have infinite immutable references
    - Why all the rules? Data safety. You don't want to be able to change a value that is being referenced by others.
- You can change traits that are used for operators. For example, `+=` is the `AddAssign` trait and can be changed.
- Its important to note the difference between statements and expressions.
    - Expressions return a value
    - Statements don't return a value and end with a semicolon
- You can return a value from loops by an expression after the `break` statement

### References 

[^MultipleValues]:, ["Does Rust have support for functions that return multiple values?"](https://stackoverflow.com/questions/18479648/does-rust-have-support-for-functions-that-return-multiple-values) stackoverflow.com https://stackoverflow.com/questions/18479648/does-rust-have-support-for-functions-that-return-multiple-values (Accessed August 13, 2022)
[^ForLoops]: , ["for loops,"](https://doc.rust-lang.org/rust-by-example/flow_control/for.html) doc.rust-lang.org. https://doc.rust-lang.org/rust-by-example/flow_control/for.html (Accessed August 20, 2022)
[^Ryan]: , Levick, Ryan ["Rust: Pass-By-Value or Pass-By-Reference?](https://blog.ryanlevick.com/rust-pass-value-or-reference/) blog.ryanlevick.com https://blog.ryanlevick.com/rust-pass-value-or-reference/ (Accessed August 21, 2022)
[^Ownership]: , ["What Is Ownership?"](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) doc.rust-lang.org https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html (Accessed August 20, 2022)