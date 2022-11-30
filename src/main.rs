fn main() {
    println!("{} World!", scope());
}

// variable s inside function scope() is not valid here

fn scope() -> &'static str {
    let s = "Hello"; // s is now valid from this point forward
                     // do stuff with s
    return s;
}

// scope()'s scope is now over, and s is no longer valid
