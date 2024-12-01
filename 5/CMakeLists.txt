cmake_minimum_required(VERSION 3.20)
project(Lab5)

set(CMAKE_C_COMPILER clang)
add_compile_options(-Wall -Wextra -Wpedantic)

add_executable(main src/main.c)
