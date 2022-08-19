// https://doc.rust-lang.org/std/fmt/trait.Display.html

fn main(){
    use std::fmt;

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    let origin = Point { x: 0, y: 0 };

    assert_eq!(format!("The origin is: {origin}"), "The origin is: (0, 0)");
}