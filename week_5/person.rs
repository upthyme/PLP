// Technically this should be in something like a src/lib.rs

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    is_vampire: bool,
}

// Originally enum was part of struct, but had weird problems compiling
#[derive(Debug)]
enum TypeOfHousing {
    Apartment,
    SingleFamilyHome,
    Condo,
    Farm,
    Rooming,
}

impl TypeOfHousing {
    fn cost_of_housing(&self) -> i32 {
        match *self {
            TypeOfHousing::Apartment => 1000 ,
            TypeOfHousing::SingleFamilyHome => 4000,
            TypeOfHousing::Condo => 1200,  
            TypeOfHousing::Farm => 8000,
            TypeOfHousing::Rooming => 700,
        }
    }
}

impl Person {
    fn make_person(name: String, age: u8, is_vampire: bool) -> Person {
        Person {
            name, 
            age, 
            is_vampire: false, 
        }
    }

    fn get_age(&self) -> &u8 {
        &self.age
    }

    fn birthday(&mut self) {
        self.age += 1;
        println!("Age: {}", self.age);
    }
}

fn main(){
    let his_name = String::from("Colin Robinson");
    let his_age = 8;
    let his_house = TypeOfHousing::Rooming;
    let vamp = true;
    let mut colin = Person { 
        name: his_name, 
        age: his_age, 
        is_vampire: vamp,
    };

    let housing_costs = his_house.cost_of_housing() ;
    println!("Your housing cost: {}", housing_costs);

    // Debug statement
    println!("{:?}", colin);
    colin.birthday();
    println!("{:?}", colin);
}