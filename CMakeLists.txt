# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.20.0)

if(${ARCH} STREQUAL "arm")
    if(CONFIG_CPU_CORTEX_M0 OR CONFIG_CPU_CORTEX_M0PLUS)
        set(target thumbv6m-none-eabi)
    else()
        message(FATAL_ERROR "CPU not supported")
    endif ()
    message(STATUS "Rust target: ${target}")
else()
    message(FATAL_ERROR "Arch ${ARCH} not supported")
endif()

if (CONFIG_USERSPACE)
    message(FATAL_ERROR "Rust doesn't support Zephyr User Mode")
endif ()

# Enable cmake support for Rust
set(RUST_CARGO_TARGET "${target}")

set(RUST_SOURCE_DIR ${APPLICATION_SOURCE_DIR})
message(STATUS "Rust source dir: ${RUST_SOURCE_DIR}")

set(RUST_BINARY_DIR ${CMAKE_CURRENT_BINARY_DIR})
message(STATUS "Rust binary dir: ${RUST_BINARY_DIR}")

if (CONFIG_RUST)
    set(RUST_LIB_NAME lib${CONFIG_RUST_APP_NAME}.a)
    message(STATUS "Rust lib name: ${RUST_LIB_NAME}")

    if (CONFIG_RUST)
        include(ExternalProject)

        ExternalProject_Add(
                rust_project
                PREFIX ${RUST_BINARY_DIR}
                SOURCE_DIR ${RUST_SOURCE_DIR}
                CONFIGURE_COMMAND ""
                BUILD_IN_SOURCE TRUE
                BUILD_ALWAYS TRUE
                BUILD_COMMAND
                "cargo"
                "build"
                "--release"
                "--target-dir=${RUST_BINARY_DIR}"
                INSTALL_COMMAND ""
                BUILD_BYPRODUCTS
                ${RUST_BINARY_DIR}/${RUST_CARGO_TARGET}/release/${RUST_LIB_NAME}
        )

        zephyr_library_import(zephyr-rs ${RUST_BINARY_DIR}/${RUST_CARGO_TARGET}/release/${RUST_LIB_NAME})
    endif ()
endif ()