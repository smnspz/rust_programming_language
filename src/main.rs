fn main() {
    let number = 2;
    println!("Is {} less then 5? {}", number, is_less_then_three(number))
}

fn is_less_then_three(number: i32) -> bool {
    if number < 5 {
        true
    } else {
        false
    }
}
