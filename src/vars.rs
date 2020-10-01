//Variables hold primitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language

pub fn run() {
    let name = "Andy";
    //mut allows your variable to be mutable/reassignable
    let mut age = 20;
    println!("My name is {} and I am {}", name, age);
    age = 21;
    println!("My name is {} and I am {}", name, age);
    //Define Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);
}
