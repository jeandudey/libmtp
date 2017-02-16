cmake_minimum_required(VERSION 3.7)
project(libmtp VERSION 0.1.0 LANGUAGES C)

set(CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/CMake/")

add_subdirectory("src/")
add_subdirectory("util/")
