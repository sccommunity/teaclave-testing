cmake_minimum_required(VERSION 3.10)

ExternalProject_Add(teaclave-sgx-sdk-assets
    GIT_REPOSITORY https://github.com/sammyne/teaclave-sgx-sdk-assets
    GIT_TAG rsgx1.1.2
    GIT_PROGRESS true
    SOURCE_DIR ${PROJECT_SOURCE_DIR}/third_party/teaclave-sgx-sdk-assets
    UPDATE_DISCONNECTED true
    CONFIGURE_COMMAND echo "skip configure for teaclave-sgx-sdk-assets"
    BUILD_COMMAND echo "skip build for teaclave-sgx-sdk-assets"
    INSTALL_COMMAND echo "skip install for teaclave-sgx-sdk-assets"
)