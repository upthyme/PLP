// main is necessary
fn ask_big(a: i8) -> bool {
    if a > 10 {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let num = 42;
    println!("num = {}", num);
    println!("Is it big? {}", ask_big(num));
  }
  