// import the 'test' macro to override the default one
use testing::{run_tests, test};

#[test]
fn hello() {
    println!("hello");
}

fn main() {
    run_tests!();
}
