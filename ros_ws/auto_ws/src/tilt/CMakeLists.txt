cmake_minimum_required(VERSION 3.8)
project(tilt)

if(CMAKE_COMPILER_IS_GNUCXX OR CMAKE_CXX_COMPILER_ID MATCHES "Clang")
  add_compile_options(-Wall -Wextra -Wpedantic)
endif()

find_package(rclcpp REQUIRED)
find_package(std_msgs REQUIRED)

add_executable(
    tilt
    src/tilt.cpp
)

ament_target_dependencies(
    tilt
    rclcpp
    std_msgs
)

install(
    TARGETS tilt
    DESTINATION lib/${PROJECT_NAME}
)

ament_package()
