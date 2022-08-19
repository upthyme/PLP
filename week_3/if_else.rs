fn main() {
    let n = 0;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n); // no new line at end
    } else {
        print!("{} is zero", n);
    }

    // Note:
    // Why does this work? Both branches MUST return the same type AND the conditionals are expressions
    // - No parentheses
    // - Followed by BLOCKS
    // - big_n is assigned either 10 * n OR n/2

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
            // TODO ^ Try suppressing this expression with a semicolon??
        };
    //   ^ Necessary semicolon!!! All `let` bindings need it.

    println!("{} -> {}", n, big_n);
}