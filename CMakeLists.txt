project(udpt)
cmake_minimum_required(VERSION 3.2)
enable_testing()

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

file(GLOB src_files "src/*.c"
        "src/*.cpp"
        "src/db/*.cpp"
        "src/http/*.cpp")

LIST(APPEND LIBS "pthread" "sqlite3" "boost_program_options" "boost_system")


add_executable(udpt ${src_files})
target_link_libraries(udpt ${LIBS})

add_executable(udpt_tests tests/main.cpp ${src_files})
target_compile_definitions(udpt_tests PRIVATE TEST=1)

target_link_libraries(udpt_tests gtest ${LIBS})

add_test(NAME udpt_tests COMMAND udpt_tests)

add_custom_target(docs
        COMMAND python -msphinx -M html . _build
        COMMAND python -msphinx -M man . _build
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}/docs
        COMMENT Building docs)
