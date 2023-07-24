mod easy {
    pub mod is_armstrong_number;
}

use easy::is_armstrong_number::is_armstrong_number;

fn main() {
    println!("is_armstrong_number(153) = {}", is_armstrong_number(1));
}
