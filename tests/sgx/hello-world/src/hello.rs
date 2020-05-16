//#[cfg(test)]
#[cfg(feature = "with-testing")]
pub mod tests {
    use testing::test;

    #[test]
    fn it_works_here_too() {
        assert_eq!(2 + 2, 4);
    }
}
