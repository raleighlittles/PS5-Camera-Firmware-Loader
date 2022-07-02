cmake_minimum_required(VERSION 3.20)
project(ps5_camera_firmware_loader DESCRIPTION "A tool for loading custom firmware onto a Playstation 5 camera using libusb")

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_BUILD_TYPE Debug)
set(CMAKE_CXX_FLAGS "-g -O0 -Wall -Wextra -Wpedantic -Werror")

include_directories("src")

add_executable(${PROJECT_NAME} main.cpp)

target_link_libraries(${PROJECT_NAME} usb-1.0)