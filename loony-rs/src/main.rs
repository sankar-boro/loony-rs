trait ImplUsername {}

struct Sankar {}

impl ImplUsername for Sankar {}

struct Username<T: ImplUsername> {
    username: T,
}

struct User {
    fname: String,
    lname: String,
    username: Username<Sankar>,
}

fn main() {
    println!("Hello, world!");
}
