cmake_minimum_required(VERSION 3.20)
project(ps5_camera_firmware_loader)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_BUILD_TYPE Debug)
set(CMAKE_CXX_FLAGS "-O0 -Wall")

include_directories("src")

add_executable(${PROJECT_NAME} main.cpp)

target_link_libraries(${PROJECT_NAME} usb-1.0)