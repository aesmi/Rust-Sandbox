use std::env;

pub fn run() {
    //This allows the program to collect arguments from the command line
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Andy";
    let status = "100%";

    // println!("Commands:{:?}", command);

    //adding control flow to cli arguments
    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command");
    }
}
