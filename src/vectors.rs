//Vectors - resizable list where elements are the same data types
use std::mem;

pub fn run() {
    //Initiated with type within angle brackets
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    //Re-assign value
    numbers[2] = 20;
    println!("{:?}", numbers);
    //Add on to vector
    numbers.push(6);
    //Pop off last value
    numbers.pop();
    //Get single val
    println!("Single Value: {}", numbers[0]);
    //Get Vector length
    println!("Vector Length: {}", numbers.len());
    //Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));
    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
    //Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    //Loop and mutate values similar to javascript map
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}
