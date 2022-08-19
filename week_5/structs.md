# Assignment 5: Structs (Classes) and Inheritance

## Discussion Questions

**1. Does your language support objects or something similar (e.g., structs)?**

Rust doesn't necessarily support objects like in Java or Python. However, Rust does support structs. 

**1a. Are there naming conventions for objects, instance variables, or functions that people writing in your language should be aware of?**

Yes, there are naming conventions. Rust will complain to you at compile time if the names don't match. 

Technically there are no objects. Instead, there are structs. Structs use UpperCamelCase.

Traits use UpperCamelCase.

Functions use snake_case. 

TODO: look at Rust instances (of structs/enums)? Are they just snake_case?

**2. Does your language have standard methods for functions that serve a similar purpose across all objects? For example, `toString()` in Java and `__str__` in Python allow information about the objects to be printed. Are there similar functions in your language?**

Rust uses traits as a abstract interface which is implemented for each type. For instance, a `String` type has a `ToString` trait, which is automatically implemented for the `Display` trait. The `Display` shows information back to the user (ie. console), and can be used with the format `fmt`.

**3. How does inheritance work (if it does)? Does your language support multiple inheritance?**

Not really. Rust uses trait objects to enable polymorphism. Note that traits are more like interfaces than classes. There is no storage of data or fields. Instead, you describe the functionality. The data would be held in a struct (or possibly a enum). Traits must have functions that can get themselves with `&self` or `&mut self` to be the respective getters and setters. 

**4. If there is inheritance, how does your language deal with overloading method names and resolving those calls?**

Conflicting? 

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
- Trait objects and dynamic dispatch
    - Specify the implementation of a polymorphic operation at run time

**5a. I know this question is vague, but objects are where languages can vary a lot, so it’s hard to ask questions that will apply to everyone**

? 

### References 
- https://doc.rust-lang.org/std/string/trait.ToString.html (Access July 2, 2022)
- https://doc.rust-lang.org/std/fmt/trait.Display.html