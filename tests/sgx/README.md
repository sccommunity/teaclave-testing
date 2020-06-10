# Hello World example without CMake

## Prerequisite
A machine or docker container with the SGX SDK and the rust toolchain installed as specified as in
official docker container [baiduxlab/sgx-rust:1804-1.1.2][1].

## Run
```bash
rm -rf build 
mkdir build
cd build
cmake ..
make
# trigger tests
make test-sgx
```

## Layout
```text
|-app           // the untrusted app to drive the enclave running
|-enclave       // the crate serving as a bridge to package the ported crate into a enclave
|-hello-world   // the ported crates to test
|-hi            // a example to test the `#[cfg(feature = "with-testing")]` feature gate of the
                // testing crate.
```

## Head Ups
- It's important for the below change to `LDFLAGS` in enclave/CMakeLists.txt from 

    ```cmake
    -L${addonLibPath} -l${rustEnclaveLib}
    ```

    to

    ```cmake
    -L${addonLibPath} \
        -Wl,--whole-archive -l${rustEnclaveLib} -Wl,--no-whole-archive \
    ```

    Related issue is discussed at [teaclave-sgx-sdk issue#232](https://github.com/apache/incubator-teaclave-sgx-sdk/issues/232#issuecomment-623804958)


[1]: https://hub.docker.com/layers/baiduxlab/sgx-rust/1804-1.1.2/images/sha256-3a2fc238147576552e9a651eeadae29e8729841d5f23b394d856c2fb64f5899c?context=explore
