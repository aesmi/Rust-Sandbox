//Tuples group together values of different types
//Max 12 elements

//You can declare tuples with types and access them by index

pub fn run(){
    let person: (&str, &str, i8) = ("Andy", "HK", 20);
    println!("{} is from {} and is {}", person.0, person.1, person.2);
}