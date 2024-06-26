# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.20.0)

if(${ARCH} STREQUAL "arm")
    if(CONFIG_CPU_CORTEX_M0 OR CONFIG_CPU_CORTEX_M0PLUS)
        set(target thumbv6m-none-eabi)
        set(clang_target thumb-v6m-unknown-none-eabi)
    else()
        message(FATAL_ERROR "CPU not supported")
    endif ()
    message(STATUS "Rust target: ${target}")
    message(STATUS "Clang target: ${clang_target}")
else()
    message(FATAL_ERROR "Arch ${ARCH} not supported")
endif()

if (CONFIG_USERSPACE)
    message(FATAL_ERROR "Rust doesn't support Zephyr User Mode")
endif ()

# Enable cmake support for Rust
set(RUST_CARGO_TARGET "${target}")
set(CLANG_TARGET "${clang_target}")

set(RUST_SOURCE_DIR ${APPLICATION_SOURCE_DIR})
message(STATUS "Rust source dir: ${RUST_SOURCE_DIR}")

set(RUST_BINARY_DIR ${CMAKE_CURRENT_BINARY_DIR})
message(STATUS "Rust binary dir: ${RUST_BINARY_DIR}")

if  (NOT ZEPHYR_BINDINGS_BINARY)
        set(ZEPHYR_BINDINGS_CRATE ${CMAKE_CURRENT_SOURCE_DIR}/tools/zephyr-bindgen)
        message(STATUS "Zephyr bindings crate: ${ZEPHYR_BINDINGS_CRATE}")

        set(ZEPHYR_BINDINGS_BUILD_DIR ${CMAKE_CURRENT_BINARY_DIR})
        message(STATUS "Zephyr bindings build dir: ${ZEPHYR_BINDINGS_BUILD_DIR}")

        set(ZEPHYR_BINDINGS_BINARY ${ZEPHYR_BINDINGS_BUILD_DIR}/release/zephyr-bindgen)

        include(ExternalProject)

        ExternalProject_Add(
                zephyr-bindings
                PREFIX ${ZEPHYR_BINDINGS_BUILD_DIR}
                SOURCE_DIR ${ZEPHYR_BINDINGS_CRATE}
                CONFIGURE_COMMAND ""
                BUILD_IN_SOURCE TRUE
                BUILD_ALWAYS TRUE
                BUILD_COMMAND
                "cargo"
                "build"
                "--release"
                "--target-dir=${ZEPHYR_BINDINGS_BUILD_DIR}"
                INSTALL_COMMAND ""
                BUILD_BYPRODUCTS
                ${ZEPHYR_BINDINGS_BINARY}
        )
endif()

set(RUST_BINDINGS_FILE ${CMAKE_CURRENT_SOURCE_DIR}/crates/zephyr-sys/src/bindings.rs)
set(ZEPHYR-SYS-CRATE ${CMAKE_CURRENT_SOURCE_DIR}/crates/zephyr-sys)

zephyr_get_include_directories_for_lang_as_string(C includes)
zephyr_get_system_include_directories_for_lang_as_string(C system_includes)
zephyr_get_compile_definitions_for_lang_as_string(C definitions)

#message(STATUS "CMSIS include directories: ${CMSIS_INCLUDE_DIRS}")

set(TARGET_CFLAGS  "${includes} ${system_include} ${definitions} -imacros ${AUTOCONF_H}")

if  (NOT EXISTS ${RUST_BINDINGS_FILE})
    message(STATUS "Rust bindings file not found: ${RUST_BINDINGS_FILE}")

    add_custom_command(
            OUTPUT ${RUST_BINDINGS_FILE}
            COMMAND ${ZEPHYR_BINDINGS_BINARY} ${ZEPHYR-SYS-CRATE} -- ${TARGET_CFLAGS} --target=${CLANG_TARGET}
            DEPENDS zephyr-bindings
    )

    add_custom_target(
            zephyr-bindings-file
            DEPENDS ${RUST_BINDINGS_FILE}
    )
endif()

if (CONFIG_RUST)
    set(RUST_LIB_NAME lib${CONFIG_RUST_APP_NAME}.a)
    message(STATUS "Rust lib name: ${RUST_LIB_NAME}")

    include(ExternalProject)

    ExternalProject_Add(
            rust_project
            PREFIX ${RUST_BINARY_DIR}
            SOURCE_DIR ${RUST_SOURCE_DIR}
            CONFIGURE_COMMAND ""
            BUILD_IN_SOURCE TRUE
            BUILD_ALWAYS TRUE
            DEPENDS zephyr-bindings-file
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