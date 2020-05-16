#![no_std]

extern crate sgx_tstd as std;

#[no_mangle]
pub extern "C" fn ecall_run_tests() -> u64 {
    wheel::tests::run();

    0
}
