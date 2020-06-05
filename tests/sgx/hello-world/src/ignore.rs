//#[cfg(test)]
#[cfg(feature = "with-testing")]
pub mod tests {
    use testing::test;

    #[test]
    #[ignore]
    fn it_should_be_ignore() {
        assert_eq!(2 + 2, 4);
    }
}
