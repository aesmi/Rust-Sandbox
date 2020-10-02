//Structs = Used to create custom data types

//Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

//Tuple Struct
//struct Color (u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

//impl are basically classes
impl Person {
    //Construct person
    //Constructor method
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    //Get full name
    //&self means reference to self
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    //Set last name
    //&mut self means reference to self but we are mutating it
    fn set_last_name(&mut self, last: &str) {
        //ensure argument is a string
        self.last_name = last.to_string();
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };
    let mut c = Color(255, 0, 0);

    c.red = 200;

    //println!("Color: {} {} {}", c.red, c.green, c.blue);

    //creating new instance of Person passing in properties as arguments
    let mut p = Person::new("John", "Doe");
    //accessing properties similar to javascript objects
    // println!("Person {} {}", p.first_name, p.last_name);

    //setting last name
    p.set_last_name("Williams");

    //call object functions
    println!("Person {} {}", p.full_name());

    //accessing tuple struct by index
    // println!("Color: {} {} {}", c.0, c.1, c.2);

}
