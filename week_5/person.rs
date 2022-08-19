fn main(){
    // technically this should be in something like a src/lib.rs
    struct Person {
        age: u8,
    }

    impl Person {
        fn birthday(&mut self) {
            self.age += 1;
            println!("Age: {}", self.age);
        }

    }
}