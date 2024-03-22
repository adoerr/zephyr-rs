# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.20.0)

find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})

project(rust_app)

target_sources(app PRIVATE src/main.c)

# Build and bring in the rust package as an "external project".
include(ExternalProject)

set(rust_app_src_dir ${PROJECT_SOURCE_DIR})
set(rust_app_build_dir ${rust_app_src_dir})

ExternalProject_Add(
        rust_app_project
        PREFIX ${standlone_build_dir}
        SOURCE_DIR ${rust_app_src_dir}
        BINARY_DIR ${rust_app_src_dir}
        CONFIGURE_COMMAND ""
        # For the build to always be run, so that changes in the Rust
        # code are detected.
        BUILD_ALWAYS TRUE
        BUILD_COMMAND
                "cargo" # "+nightly"
                 "build"
                 "--target" "thumbv6m-none-eabi"
                  #"--release"
        INSTALL_COMMAND ""
        BUILD_BYPRODUCTS
          ${rust_app_build_dir}/target/thumbv6m-none-eabi/release/libhello_world.a
)

add_library(rust_app_lib STATIC IMPORTED GLOBAL)

add_dependencies(
        rust_app_lib
        rust_app_project
)

set_target_properties(rust_app_lib PROPERTIES IMPORTED_LOCATION
  ${rust_app_build_dir}/target/thumbv6m-none-eabi/debug/libhello_world.a)

# Add our application, along with -lgcc
target_link_libraries(app PUBLIC rust_app_lib gcc)