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

