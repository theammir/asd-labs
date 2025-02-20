cmake_minimum_required(VERSION 3.20)
project(Modular1)

set(CMAKE_C_COMPILER clang)
add_compile_options(-Wall -Wextra -Wpedantic -trigraphs)

add_executable(main src/main.c)
