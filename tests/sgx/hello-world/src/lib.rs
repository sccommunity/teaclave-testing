#![no_std]

extern crate sgx_tstd as std;

//#[cfg(test)]
#[cfg(feature = "with-testing")]
pub mod tests {
    use testing::test;

    pub fn run() {
        testing::run();
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
