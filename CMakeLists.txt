cmake_minimum_required(VERSION 3.10)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(CppLC LANGUAGES CXX VERSION 0.0.1)

find_package(Catch2 REQUIRED)

add_subdirectory(cpp)

