#![no_std]

extern crate sgx_tstd as std;

#[no_mangle]
pub extern "C" fn ecall_run_tests() -> u64 {
    // @TODO: refactor run to return the true
    if wheel::tests::run() {
        0
    } else {
        1
    }
}
