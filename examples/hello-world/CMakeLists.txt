# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.20.0)

get_filename_component(ZEPHYR_RUST ${CMAKE_CURRENT_SOURCE_DIR}/../.. ABSOLUTE)

list(APPEND EXTRA_ZEPHYR_MODULES ${ZEPHYR_RUST})

message(STATUS "Zephyr Rust module: ${ZEPHYR_RUST}")

find_package(Zephyr)

project(hello_world)

target_sources(app PRIVATE ./src/main.c)