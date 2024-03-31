# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.20.0)

if(${ARCH} STREQUAL "arm")
    if(CONFIG_CPU_CORTEX_M0 OR CONFIG_CPU_CORTEX_M0PLUS)
        set(target thumbv6m-none-eabi)
    else()
        message(FATAL_ERROR "CPU not supported")
    endif ()
    message(STATUS "Rust Target: ${target}")
else()
    message(FATAL_ERROR "Arch ${ARCH} not supported")
endif()

# Enable cmake support for Rust
set(Rust_CARGO_TARGET "${target}")

include(FetchContent)

FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.4
)

FetchContent_MakeAvailable(Corrosion)

set(rust_app_build ${CMAKE_CURRENT_BINARY_DIR}/app)
message(STATUS "Rust Build: ${rust_app_build}")

if (CONFIG_RUST)
    include(ExternalProject)

    ExternalProject_Add(
            rust_project
            PREFIX ${CMAKE_CURRENT_BINARY_DIR}}
            SOURCE_DIR ${CMAKE_CURRENT_SOURCE_DIR}
            CONFIGURE_COMMAND ""
            BUILD_IN_SOURCE TRUE
            BUILD_ALWAYS TRUE
            BUILD_COMMAND
            "cargo"
            "build"
            "--release"
            "--target-dir=${rust_app_build}"
            INSTALL_COMMAND ""
            BUILD_BYPRODUCTS
            ${rust_app_build}//thumbv6m-none-eabi/release/libhello_world.a
    )

    zephyr_library_import(zephyr_sys ${rust_app_build}//thumbv6m-none-eabi/release/libhello_world.a)
endif ()