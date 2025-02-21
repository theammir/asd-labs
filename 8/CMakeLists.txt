cmake_minimum_required(VERSION 3.20)
project(Lab8)

set(CMAKE_C_COMPILER clang)
add_compile_options(-Wall -Wextra -Wpedantic)

add_library(input STATIC src/input.c)
add_library(linkedlist STATIC src/linkedlist.c)

add_executable(main src/main.c)
target_link_libraries(main PRIVATE input)
target_link_libraries(main PRIVATE linkedlist)
