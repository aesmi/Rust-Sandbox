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
    //i32 integer 32 bit max(~2.1b)
    const ID: i32 = 001;
    println!("ID: {}", ID);
    //Assign multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}
