# Hello World example without CMake

## Note
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
