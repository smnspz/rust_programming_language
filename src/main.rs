fn main() {
    let number = 2;
    println!("Is {} less then 5? {}", number, is_less_then_three(number));
    println!("2.5 + 2.5 equals 5! {}", expressions_assignment(true));
}

fn is_less_then_three(number: i32) -> bool {
    if number < 5 {
        true
    } else {
        false
    }
}

fn expressions_assignment(condition: bool) -> i32 {
    let number = if condition { 5 } else { 6 };

    return number;
}
