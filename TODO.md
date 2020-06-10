# TODO
- support tests filtering, which will add the `x filtered` to test result
- figure out how to re-export macros in the inventory so as to eliminate requirements of dependency
on inventory for each enclave crates.

# DONE
- 2020/06/04: `should_panic` macro
- 2020/06/05: implement the `ignore` macro
- 2020/06/10: add the feature gate `#[cfg(feature = "with-testing")]` to address the case of test
functions spread around rather than being collected within a single module
