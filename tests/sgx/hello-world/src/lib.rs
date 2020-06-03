#![no_std]

extern crate sgx_tstd as std;

mod hello;
mod panicking;

//#[cfg(test)]
#[cfg(feature = "with-testing")]
pub mod tests {
    use testing::{generate_runner, test};

    generate_runner!();

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
