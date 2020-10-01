//public function
pub fn run() {
    //Print to console
    println!("Hello, world!");
    //string literals should be wrapped in brackets and passed as an argument
    println!("Number: {}", 1);
    //Basic Format
    println!("{} is form {}", "Andy", "HK");
    //Which arg is passed can be changed by dictating the index
    println!("{0} is form {1} and {0} likes to {2}", "Andy", "HK", "code");
    //Named Arguments
    println!(
        "{name} likes to play {activitiy}",
        name = "Andy",
        activitiy = "Valorant"
    );
    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));
    //Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
