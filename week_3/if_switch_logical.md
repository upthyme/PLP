# Assignment 3: If/else, switch, logical operators

## Discussion Questions

**1. What are the boolean values in your language? (e.g., True and False, true and false, 1, and 0, etc)**

The boolean type `bool` represents a value that can only be `true` or `false`. If casted to a integer, the `true` is 1 and `false` is 0.

**2. What types of conditional statements are available in your language? (if/else, if/then/else, if/elseif/else). Does your language allow for statements other than “if” (for example, Perl has an “unless” statement, which does the opposite of “if”!)**

The following conditional statements are available: 
- if/else if/else
- if let

I would also consider `assert!` to be an important macro that can use the `bool` type in testing conditions. (check?)

**3. Does your language use short-circuit evaluation? Include an example of the short-circuit logic working or not working (or both, if your language is like Java and supports both!)**

Yes. There is an example of bitwise operators. 
- Bitwise AND (&)
- Bitwise OR (|)
- Bitwise XOR (^)
- Bitwise NOT (!)
- Left Shift (<<)
- Right Shift (>>)

**4. How does your programming language deal with the “dangling else” problem?**

**5. If your language supports switch or case statements, do you have to use “break” to get out of them? Can you use “continue” to have all of them evaluated?**

Rust supports `match`, which matches a pattern.

### References 
- [^PrimitiveTypebool]: ["Primitive Type bool,"](https://doc.rust-lang.org/std/primitive.bool.html) docs.rust-lang.org. https://doc.rust-lang.org/std/primitive.bool.html (Accessed June 27, 2022)