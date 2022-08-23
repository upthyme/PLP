# Assignment 5: Structs (Classes) and Inheritance

## Discussion Questions

**1. Does your language support objects or something similar (e.g., structs)?**

Rust doesn't necessarily support objects like Java or Python. However, Rust does support structs and enums that can be used to model objects. Both structs and enums are custom and user defined. 

Structs are a collection-like data type that can group different data types. Structs are similar to tuples. Both can hold multiple values of different types. However, structs can identify each of its values with names. The combination of name with value is called a _field_. The result is that we can easily define a `impl` or implementation for the struct we make. This can be used to create class OOP setters and getters of instances.

Below is an example of a struct. Inside the curly brackets, the fields are defined. 

``` rust 
struct Dog {
    name: String,
    age: u8,
    food: String,
    bark: String,
}

```

Enums are a type that _enumates_ different variants. This is useful when trying to represent what _kind_ something can be ^[Enum]. 

Below is an example of an enum that is representing a possible set of values for a coin. In our representation, a coin can be either a quarter, dime, or penny.

``` rust
enum Coin {
    Quarter,
    Dime,
    Penny,
}
```

If you combine a struct and a enum, you can get more functionality.

For instance, what if we wanted to define the types of barking our `Dog` struct could have? We could have the bark name be tied to a variant in a enum.

``` rust 
struct Dog {
    ...
    bark: DogNoise 
}

enum DogNoise {
    woof,
    bark,
    hoo,
    whimper,
    purrrr,
    grrr,
}

```

**1a. Are there naming conventions for objects, instance variables, or functions that people writing in your language should be aware of?**

Yes, there are naming conventions. Rust will complain to you at compile time if the names don't match. Although the code will still compile, you will get warnings. 

For the "type level" constructs, CamelCase is preferred. This is when we are dealing with types or traits [^Naming].

Technically there are no objects. However, there are instances of a data type. These are usually in snake_case because they are "value level". 

- Structs use UpperCamelCase
- Enum variants uses CamelCase 
- Types use CamelCase 
- Traits use UpperCamelCase
- Methods use snake_case
- Functions use snake_case
- Local variables use snake_case

**2. Does your language have standard methods for functions that serve a similar purpose across all objects? For example, `toString()` in Java and `__str__` in Python allow information about the objects to be printed. Are there similar functions in your language?**

Rust uses traits as a abstract interface which is implemented for each type. For instance, a `String` type has a `ToString` trait, which is automatically implemented for the `Display` trait. The `Display` trait shows information back to the user (ie. console), and can be used with the format `fmt` [^Trait].

All the primitive data types in Rust have a `Display` trait. However, if you want to print out a custom defined object, you need to define it in the implementation.

**3. How does inheritance work (if it does)? Does your language support multiple inheritance?**

Inheritance is not supported or generally used in Rust. There just isn't a supported concept for it in the docs. 

However, you can still model it. Rust can use trait objects to enable polymorphism. Note that traits are more like interfaces than classes. There is no storage of data or fields. Instead, you describe the functionality. The data would be held in a struct (or possibly a enum). Traits must have functions that can get themselves with `&self` or `&mut self` to be the respective getters and setters. 

Here is a tutorial[^Tutorial] that shows how to model inheritance. A interface or `trait` is used to define the relationship between two data types in rust. 

**4. If there is inheritance, how does your language deal with overloading method names and resolving those calls?**

Technically, you can do functional overloading with a `trait`[^StackOverflow]

Below we define a trait, but we also have three implementations for three different types. For example, if you call `pink.CountMoney()` and the variable `pink` is a `piggybank`, then the method implementation for the `piggybank` type will be chosen.

``` rust
trait CountMoney {
    ...
}

impl CountMoney for u32 {
    ...
}

impl CountMoney for piggybank {
    ...
}

impl CountMoney for safe {
    ...
}

```
**5. Is there anything else that’s important to know about objects and inheritance in your language?**

- A *state pattern* is an object oriented design pattern. There is a set of *state objects* and the value's behavior changes based on the internal state.
- Rust models things with "has-a" relationship (ie. composition) instead of "is-a" relationships (ie. inheritance). 
- The `impls` are used to define methods
- enums can be used in place of structs
- `self`, `&self`, and `&mut self` are all different
    - `self` is a value on the stack (and takes ownership)
    - `&self`  is a reference to self 
    - `&mut self` is a mutable reference
- Associated functions are the same as static methods (ie. you can use them without creating an object)
- Methods are functions that should work on a instance of the struct
- Trait objects and dynamic dispatch
    - Need to specify the implementation of a polymorphic operation at run time

### References 

[^Enum]: , [Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html) doc.rust-lang.org https://doc.rust-lang.org/rust-by-example/custom_types/enum.html (Accessed August 22, 2022)
[^Naming]: , [Naming Conventions](https://doc.rust-lang.org/1.0.0/style/style/naming/README.html) doc.rust-lang.org https://doc.rust-lang.org/1.0.0/style/style/naming/README.html (Accessed August 22, 2022)
[^Trait]: , [Trait std::string::ToString](https://doc.rust-lang.org/std/string/trait.ToString.html) doc.rust-lang.org https://doc.rust-lang.org/std/string/trait.ToString.html (Accessed July 2, 2022)
[^Tutorial]: , [Inheritance with Traits](https://riptutorial.com/rust/example/22917/inheritance-with-traits) riptutorial.org https://riptutorial.com/rust/example/22917/inheritance-with-traits (Accessed August 22, 2022)
[^StackOverflow]: , [Why doesn't Rust support overloading function or method?](https://stackoverflow.com/questions/56508697/why-doesnt-rust-support-overloading-function-or-method) stackoverflow https://stackoverflow.com/questions/56508697/why-doesnt-rust-support-overloading-function-or-method (Accessed August 22, 2022)