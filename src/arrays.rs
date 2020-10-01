//Arrays - fixed list where elements are the same data types

pub fn run(){
    //Initiated with type and then length
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", numbers);
    //Get single val
    println!("Single Value: {}", numbers[0]);
    //Get array length
    println!("Array Length: {}", numbers.len());
    //Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
}