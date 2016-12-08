project(udpt)
cmake_minimum_required(VERSION 3.6)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
add_definitions(-DBOOST_LOG_DYN_LINK)

file(GLOB src_files "src/*.c"
        "src/*.cpp"
        "src/db/*.cpp"
        "src/http/*.cpp")


add_executable(udpt ${src_files})
target_link_libraries(udpt pthread sqlite3 boost_log boost_program_options boost_thread boost_system)
