cmake_minimum_required(VERSION 3.20)
project(Lab2)

set(CMAKE_C_COMPILER clang)
add_compile_options(-Wall -Wextra -Wpedantic)

add_executable(a src/a.c)
add_executable(b src/b.c)

find_library(MATH_LIBRARY m)
if(MATH_LIBRARY)
	target_link_libraries(a PUBLIC ${MATH_LIBRARY})
	target_link_libraries(b PUBLIC ${MATH_LIBRARY})
endif()
