cmake_minimum_required(VERSION 3.20)
project(Lab1)

set(CMAKE_C_COMPILER clang)
add_compile_options(-Wall -Wextra -Wpedantic -trigraphs)

add_executable(a src/a.c)
add_executable(b src/b.c)
