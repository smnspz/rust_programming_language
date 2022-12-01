fn main() {
    println!("{} World!", scope());

    let s1 = gives_ownership(); // moves return value into s1

    let s2: String = String::from("Hello"); // s2 comes into scope

    let _s3: String = takes_and_gives_back(s2);

    let mut s4: String = String::from("Hello");

    // how to pass a mutable variable reference into a function
    change(&mut s4);

    println!("This string was changed: {}", s4);

    // with the ampersand we pass a reference to the value
    // so that we can still use the variable afterwards
    let _length = calculate_length(&s1);

    // let reference_to_nothing = dangle();

    let word = first_word(&s4);

    string_slice();
}

// variable s inside function scope() is not valid here

fn scope() -> &'static str {
    let s = "Hello"; // s is now valid from this point forward
    s
}

// scope()'s scope is now over, and s is no longer valid

fn _strings() {
    // Allocates this type on the heap, can be useful
    // when the amount of text is unknown at compile time
    let mut s = String::from("Hello");
    // This kind of string can be mutated, for example:
    s.push_str(", World!");

    println!("{}", s);
    // Rust automatically frees up the memory utilized by s
    // when it goes out of scope.
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello"); // comes into scope
    some_string // and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string moves into scope and moves out to the calling function
}

// borrowing = having references as functions parameters
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

// creates a dangling reference
// fn dangle() -> &String {
//     let s = String::from("Hello"); // new string
//     &s // returns a reference to the string
// } // Here the string goes out of scope, it points to an invalid string

fn no_dangle() -> String {
    let s = String::from("Hello");
    s
}

fn first_word(s: &String) -> usize {
   let bytes = s.as_bytes();

   for (i, &item) in bytes.iter().enumerate() {
       if item == b' ' {
           return i;
       }
   }

   s.len()
}

fn string_slice() {
    let hello_world: String = String::from("Hello World");
    let hello = &hello_world[0..5];
    let world = &hello_world[6..11];
    println!("{} {}", hello, world);
}
