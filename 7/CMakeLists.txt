cmake_minimum_required(VERSION 3.20)
project(Lab7)

set(CMAKE_C_COMPILER clang)
add_compile_options(-Wall -Wextra -Wpedantic)

add_library(input STATIC src/input.c)

foreach(target linear a b c)
  add_executable(${target} src/${target}.c)
  target_link_libraries(${target} PRIVATE input)
endforeach()
