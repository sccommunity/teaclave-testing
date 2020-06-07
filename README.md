# teaclave-testing 

![build](https://github.com/sammyne/teaclave-testing/workflows/build/badge.svg?branch=rsgx1.1.2)

A crate helps to testing enclaves written based on the [teaclave-sgx-sdk][1].

## Why
The official [sgx_unittest][4] helps us to write unit tests for crates ported to teaclave-sgx-sdk.
The library crate is a great tool. The bad thing is that I'm too lazy to do the manual chores when
using it in practice like official [samplecode][7]:( However, it can be bettered as suggested by
[teaclave-sgx-sdk issue#232][3]. Thanks to the prototype implemented by [mssum][8], here comes my
implementation.

When porting crates to teaclave-sgx-sdk, we should minimize the change to unit tests of the ported
crates. That's to say, following macros/attributes should remain untouched and function as close as
that of the official rust
- `#[test]`
- `#[should_panic(expected = "your expected panicking message")]`
- `#[ignore]`

However, it has following limits I find no way to address for now
- `#[should_panic(expected = "...")]` and `#[ignore]` must come after `#[test]` but not before to
take effects.
- The `generate_runner!();` macro has to declared the exported `tests` under the crate root

## Environment
Currently, the supported version of teaclave-sgx-sdk is [v1.1.2][2] only.

## Examples
Please check the [source code](./tests/sgx/hello-world/src/lib.rs) and [docs](./tests/sgx/README.md) to
see the crate's usage.

> FYI: To avoid optimization (which will prune away some test cases) by the compiler, solution
> proposed by [teaclave-sgx-sdk issue#232][6] needs taking.

## Recommendation
- [cargo-teaclave][9]: A tool helps to test teaclave-sgx-sdk-ported crates by generating testing drivers automatically

## References
- [teaclave-sgx-sdk issue#232][3]

[1]: https://github.com/apache/incubator-teaclave-sgx-sdk
[2]: https://github.com/apache/incubator-teaclave-sgx-sdk/tree/v1.1.2
[3]: https://github.com/apache/incubator-teaclave-sgx-sdk/issues/232
[4]: https://github.com/apache/incubator-teaclave-sgx-sdk/tree/v1.1.2/sgx_tunittest
[5]: https://github.com/apache/incubator-teaclave-sgx-sdk/blob/v1.1.2/samplecode/unit-test/enclave/src/lib.rs
[6]: https://github.com/apache/incubator-teaclave-sgx-sdk/issues/232#issuecomment-623804958
[7]: https://github.com/apache/incubator-teaclave-sgx-sdk/blob/v1.1.2/samplecode/unit-test/enclave/src/lib.rs
[8]: https://github.com/mssun
[9]: https://github.com/sammyne/cargo-teaclave

