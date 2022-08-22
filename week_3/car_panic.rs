// This code will run but it'll panic!
// Demonstrates short circuit evaluation

fn main(){
    let has_car: bool = false;
    let can_drive = has_car || panic!(); 
}