cmake_minimum_required(VERSION 3.20)
project(Lab3)

set(CMAKE_C_COMPILER clang)
add_compile_options(-Wall -Wextra -Wpedantic)

add_executable(main src/main.c)

find_library(MATH_LIBRARY m)
if(MATH_LIBRARY)
	target_link_libraries(main PUBLIC ${MATH_LIBRARY})
endif()

