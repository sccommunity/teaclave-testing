//#[cfg(test)]
#[cfg(feature = "with-testing")]
pub mod tests {
    use testing::test;

    #[test]
    #[should_panic]
    fn it_should_panic() {
        panic!("I'm panicking");
    }

    #[test]
    #[should_panic(expected = "I'm")]
    fn it_should_panic_and_expect_something() {
        panic!("I'm panicking");
    }

    #[test]
    #[should_panic(expected = "\"panicking\"")]
    fn it_should_panic_and_expect_something_with_double_quote() {
        panic!("I'm \"panicking\"");
    }

    #[test]
    #[should_panic(expected = "I'm a long long long long long long long long long long panic")]
    fn long_long_should_panic() {
        panic!("I'm a long long long long long long long long long long panic");
    }

    //#[test]
    //#[should_panic(expected = "I'm hello")]
    //fn it_should_panic_and_expect_something_failed() {
    //    panic!("I'm panicking");
    //}
}
