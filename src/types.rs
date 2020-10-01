/*
Primitive Types
u is for unsigned
i is for signed
unsignsed just mean no negative values, essentially no signs
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

//Rust is a statically typed language, which means that it must know the types of all variables at compile time, however,

pub fn run() {
    //Default is "i32" for integers
    let x = 1;
    //Default is "f64" for floats
    let y = 2.5;
    //Add explicit type
    let z: i64 = 13213213213213;
    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    //Boolean
    let is_active: bool = true;
    //Get boolean from expression
    let is_greater: bool = 10 > 5;
    //character literals are '
    let a1 = 'a';
    //you can use unicode for emojis
    let face = '\u{1F600}';
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
