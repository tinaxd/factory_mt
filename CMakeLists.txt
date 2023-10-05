cmake_minimum_required(VERSION 3.0)

project(factory)

file(GLOB_RECURSE SOURCES "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp" "${CMAKE_CURRENT_SOURCE_DIR}/src/*.c")
file(GLOB_RECURSE HEADERS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.h")

add_custom_command(
    OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/lex.yy.c"
    COMMAND flex -o "${CMAKE_CURRENT_BINARY_DIR}/lex.yy.c" "${CMAKE_CURRENT_SOURCE_DIR}/src/factory.l"
    DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/factory.l" "${CMAKE_CURRENT_BINARY_DIR}/factory.tab.h"
)

add_custom_command(
    OUTPUT "${CMAKE_CURRENT_BINARY_DIR}/factory.tab.c" "${CMAKE_CURRENT_BINARY_DIR}/factory.tab.h" "${CMAKE_CURRENT_BINARY_DIR}/parser.log"
    COMMAND bison -d -r all --report-file="${CMAKE_CURRENT_BINARY_DIR}/parser.log" -o "${CMAKE_CURRENT_BINARY_DIR}/factory.tab.c" "${CMAKE_CURRENT_SOURCE_DIR}/src/factory.y"
    DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/factory.y"
)

add_executable(factory ${SOURCES} "${CMAKE_CURRENT_BINARY_DIR}/lex.yy.c" "${CMAKE_CURRENT_BINARY_DIR}/factory.tab.c")
target_compile_options(factory PRIVATE -Wall -Wextra -O0 -g3)
target_include_directories(factory PRIVATE "src" "${CMAKE_CURRENT_BINARY_DIR}")
