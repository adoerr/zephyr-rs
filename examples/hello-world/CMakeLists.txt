# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.20.0)

find_package(Zephyr REQUIRED HINTS $ENV{ZEPHYR_BASE})

project(hello-world)

target_sources(app PRIVATE src/main.c)

if(CMAKE_BUILD_TYPE MATCHES Debug)
    set(LIB_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../target/thumbv6m-none-eabi/debug)
else()
    set(LIB_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../target/thumbv6m-none-eabi/release)
endif()

add_library(hello_world STATIC IMPORTED GLOBAL)

set_target_properties(hello_world PROPERTIES IMPORTED_LOCATION ${LIB_DIR}/libhello_world.a)

target_link_libraries(app PUBLIC hello_world)