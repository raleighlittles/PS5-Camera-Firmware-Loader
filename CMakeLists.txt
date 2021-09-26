cmake_minimum_required(VERSION 3.20)
project(ps5_camera)

set(CMAKE_CXX_STANDARD 23)

add_executable(ps5_camera main.cpp)

target_link_libraries(ps5_camera usb-1.0)