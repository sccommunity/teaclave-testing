# TODO
- support tests filtering, which will add the `x filtered` to test result
- figure out how to re-export macros in the inventory so as to eliminate requirements of dependency
on inventory for each enclave crates.
- downgrade the `regex` crate depended by our local `proc_macro` to make it distinct from the
variants already ported to [teaclave-sgx-sdk][1] by vendor such as [mesalock-linux][2]

# DONE
- 2020/06/04: `should_panic` macro
- 2020/06/05: implement the `ignore` macro
- 2020/06/10: add the feature gate `#[cfg(feature = "with-testing")]` to address the case of test
functions spread around rather than being collected within a single module

[1]: https://github.com/apache/incubator-teaclave-sgx-sdk
[2]: https://github.com/mesalock-linux/regex-sgx
