fn main() {
    another_function(five());
    expressions()
}
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn expressions() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5 // returns 5 without the need to use the return keyword
}