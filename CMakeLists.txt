project(udpt)
cmake_minimum_required(VERSION 3.2)
enable_testing()

set(CMAKE_BUILD_TYPE Release)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

file(GLOB src_files "src/*.c"
        "src/*.cpp"
        "src/db/*.cpp")

if(CMAKE_SYSTEM_NAME STREQUAL FreeBSD)
    include_directories("/usr/include" "/usr/local/include")
endif()

find_library(PTHREADS NAMES "pthread")
find_library(SQLITE NAMES "sqlite3" "libsqlite" "libsqlite3")
find_library(BOOST_PROG_OPTS NAMES "boost_program_options")
find_library(BOOST_SYSTEM NAMES "boost_system")
find_library(LIBEVENT NAMES "event")
find_library(LIBEVENTP NAMES "event_pthreads")
find_library(GTEST NAMES "gtest")

LIST(APPEND LIBS ${PTHREADS} ${SQLITE} ${BOOST_PROG_OPTS} ${BOOST_SYSTEM} ${LIBEVENT} ${LIBEVENTP})
add_definitions(-Wall)

add_executable(udpt ${src_files})
target_link_libraries(udpt ${LIBS})

add_executable(udpt_tests EXCLUDE_FROM_ALL tests/main.cpp ${src_files})
target_compile_definitions(udpt_tests PRIVATE TEST=1)

target_link_libraries(udpt_tests ${GTEST} ${LIBS})

add_test(NAME udpt_tests COMMAND udpt_tests)

# Add version to code
execute_process(
        COMMAND git log -1 --format="%h"
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        OUTPUT_VARIABLE GIT_COMMIT
        OUTPUT_STRIP_TRAILING_WHITESPACE
)
configure_file(
        ${CMAKE_SOURCE_DIR}/src/version.h.in
        ${CMAKE_CURRENT_BINARY_DIR}/generated/version.h
)
include_directories(${CMAKE_CURRENT_BINARY_DIR}/generated)

add_custom_target(docs
        COMMAND python -msphinx -M html . _build
        COMMAND python -msphinx -M man . _build
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/docs
        COMMENT Building docs)
