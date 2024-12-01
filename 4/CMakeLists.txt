cmake_minimum_required(VERSION 3.20)
project(Lab4)

set(CMAKE_C_COMPILER clang)
add_compile_options(-Wall -Wextra -Wpedantic)

add_executable(main src/main.c)
find_package(Curses REQUIRED)
target_link_libraries(main ncurses)
