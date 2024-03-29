//Enums are types which have a few definite values

enum Movement {
    //Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    //Perform action depending on info
    //Similar to switch in Javascript
    match m {
        //arrow returns a function to be executed
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}

pub fn run() {
    let avatar1 = Movement::Up;
    let avatar2 = Movement::Down;
    let avatar3 = Movement::Left;
    let avatar4 = Movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
