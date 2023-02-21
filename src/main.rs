use crate::rover::rover::Rover;

mod rover;

fn main() {
    println!("Hello, world!");
    let t = Rover::new(2, vec![vec!["".to_string(); 2]; 3], (10, 20));
}
