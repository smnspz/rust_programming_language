fn main() {
    let number = 2;
    println!("Is {} less then 5? {}", number, is_less_then_three(number));
    println!("2.5 + 2.5 equals 5! {}", expressions_assignment(true));
    loop_until_result();
    for_loop_example();
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

fn loop_until_result() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("Increased counter value to: {}", counter);
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {}", result);
}

fn for_loop_example() {
    let a = [10, 20, 30, 40, 50, 60, 70];

    for element in a.iter() {
        println!(
            "This is the value of each element of the array: {}",
            element
        )
    }
}
